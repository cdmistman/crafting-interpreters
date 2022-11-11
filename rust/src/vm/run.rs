use std::ops::Neg;

use eyre::Result;

use super::*;
use crate::chunk::Bytecode;
use crate::mem::GcRef;
use crate::obj::ObjClass;
use crate::obj::ObjClosure;
use crate::obj::ObjFunction;
use crate::obj::ObjInstance;
use crate::obj::ObjNative;

impl<const MAX_FRAMES: usize, const STACK_SIZE: usize>
	Vm<MAX_FRAMES, STACK_SIZE>
{
	fn run(&mut self) -> Result<()> {
		if cfg!(feature = "debug-trace") {
			let mut slots = self.stack.iter();
			slots.next().map(|slot| print!("[ {slot} ]"));
			slots.for_each(|slot| print!(" [ {slot} ]"));
			println!();
			// TODO: disassembleInstruction
		}

		loop {
			use crate::chunk::Op::*;

			let bytecode = self.read_byte();
			match unsafe { bytecode.instr } {
				Closure => {
					let Some(function) = self
						.read_constant()
						.as_casted_obj::<ObjFunction>()
					else {
						return Err(eyre::eyre!("Only function objects can become a closure"));
					};
					let mut closure = ObjClosure::new(function);
					self.push(closure.value());

					for ii in 0..closure.upvalues.len() {
						let (is_local, slot_index) = unsafe {
							(self.read_byte().index, self.read_byte().index)
						};
						closure.upvalues[ii] = if is_local > 0 {
							let start_slot = self.frame().slots.cast::<Value>();
							let upvalues = start_slot.map_addr(|addr| {
								addr.checked_add(slot_index as _).unwrap()
							});

							self.capture_upvalues(upvalues)
						} else {
							self.frame().closure.upvalues[ii]
						};
					}
				},
				Constant => {
					let constant = self.read_constant();
					self.push(constant)
				},
				False => self.push(Value::Bool(false)),
				Nil => self.push(Value::Nil()),
				True => self.push(Value::Bool(true)),

				Call => {
					let arg_count = self.read_byte();
					let arg_count = unsafe { arg_count.index };
					self.call_value(self.peek(arg_count as _), arg_count)?;
				},
				Jump => {
					let offset = self.read_short();
					let frame = self.frame_mut();
					frame.ip = frame.ip.map_addr(|ip| ip + (offset as usize));
				},
				JumpIfFalse => {
					let offset = self.read_short();
					if self.peek(0).is_falsey() {
						let frame = self.frame_mut();
						frame.ip =
							frame.ip.map_addr(|ip| ip + (offset as usize));
					}
				},
				Loop => {
					let offset = self.read_short();
					let frame = self.frame_mut();
					frame.ip = frame.ip.map_addr(|ip| ip - (offset as usize));
				},
				Pop => drop(self.pop()),

				DefineGlobal => {
					let name = self.read_string();
					let value = self.peek(0);
					self.globals.insert(name, value);
					self.pop();
				},
				GetGlobal => {
					let name = self.read_string();
					let Some(value) = self.globals.get(&name) else {
						return Err(eyre!("Undefined variable '{name}'."));
					};
					self.push(*value)
				},
				SetGlobal => {
					let name = self.read_string();
					let value = self.peek(0);
					if let None = self.globals.insert(name, value) {
						self.globals.remove(&name);
						return Err(eyre!("Undefined variable '{name}'."));
					}
				},
				GetLocal => {
					let slot = self.read_byte();
					let value = unsafe {
						let slot = slot.index;
						self.frame().slots.as_ref()[slot as usize].assume_init()
					};
					self.push(value)
				},
				SetLocal => {
					let slot = self.read_byte();
					let value = self.peek(0);
					let mut slots = self.frame_mut().slots;
					let slot = unsafe {
						let slots = slots.as_mut();
						slots.get_unchecked_mut(slot.index as usize)
					};
					slot.write(value);
				},
				GetProperty => {
					let Some(instance) = self
						.peek(0)
						.as_casted_obj::<ObjInstance>()
					else {
						return Err(eyre!("Only instances have properties."));
					};
					let name = self.read_string();

					let Some(value) = instance.fields.get(&name) else {
						return Err(eyre!("Undefined property '{name}'."));
					};

					self.pop(); // instance
					self.push(*value);
				},
				CloseUpvalue => {
					self.close_upvalues(&self.stack[self.stack.len() - 1] as _)
				},
				GetUpvalue => {
					let slot = self.read_byte();
					let slot = unsafe { slot.index };
					let value =
						self.frame().closure.upvalues[slot as usize].location;
					self.push(unsafe { value.as_ref() }.clone());
				},
				SetProperty => {
					let Some(mut instance) = self
						.peek(1)
						.as_casted_obj::<ObjInstance>()
					else {
						return Err(eyre!("Only instances have properties."));
					};
					let name = self.read_string();
					let value = self.peek(0);
					instance.fields.insert(name, value);

					let value = self.pop();
					self.pop();
					self.push(value);
				},

				Equal => {
					let a = self.pop();
					let b = self.pop();
					self.push(Value::Bool(a == b));
				},
				Less => self.binary_op(|a, b| Value::Bool(a < b)),
				Greater => self.binary_op(|a, b| Value::Bool(a > b)),

				Add => {
					let l = self.peek(0);
					let r = self.peek(1);
					if let Some(a) = l.as_casted_obj::<ObjString>()
						&& let Some(b) = r.as_casted_obj::<ObjString>()
					{
						let string = ObjString::concat(a, b);
						self.push(string.value());
					} else if l.is_number() && r.is_number() {
						self.binary_op(|a, b| Value::Number(a + b))
					} else {
						return Err(eyre!("Operands must be two numbers or two strings."));
					}
				},
				Divide => self.binary_op(|a, b| Value::Number(a / b)),
				Multiply => self.binary_op(|a, b| Value::Number(a * b)),
				Negate => {
					let Some(val) = self.peek(0).as_number() else {
						return Err(eyre!("Operand must be a number."));
					};
					self.pop();
					self.push(Value::Number(val.neg()));
				},
				Subtract => self.binary_op(|a, b| Value::Number(a - b)),

				Not => {
					let is_falsey = self.pop().is_falsey();
					self.push(Value::Bool(is_falsey));
				},

				Print => println!("{}", self.pop()),
				_ => todo!(),
			}
		}
	}
}

// trait for restricting scoping of these methods
trait RunUtil {
	fn call(&mut self, closure: GcRef<ObjClosure>, arg_count: u8)
		-> Result<()>;

	fn call_value(&mut self, callee: Value, arg_count: u8) -> Result<()>;

	fn capture_upvalues(&mut self, values: NonNull<Value>)
		-> GcRef<ObjUpvalue>;

	fn binary_op(&mut self, op: impl FnOnce(f64, f64) -> Value);

	fn close_upvalues(&mut self, last: *const Value);

	fn frame<'frame, 'vm: 'frame>(&'vm self) -> &'frame CallFrame;

	fn frame_mut<'frame, 'vm: 'frame>(&'vm mut self) -> &'frame mut CallFrame;

	fn new_instance(&mut self, klass: GcRef<ObjClass>) -> GcRef<ObjInstance>;

	fn read_byte(&mut self) -> Bytecode;

	fn read_constant(&mut self) -> Value;

	fn read_short(&mut self) -> u16;

	fn read_string(&mut self) -> GcRef<ObjString>;

	unsafe fn stack_slice_from_top<'slice, 'me: 'slice>(
		&'me self,
		len: usize,
	) -> &'slice [Value];
}

impl<const MAX_FRAMES: usize, const STACK_SIZE: usize> RunUtil
	for Vm<MAX_FRAMES, STACK_SIZE>
{
	fn call(
		&mut self,
		closure: GcRef<ObjClosure>,
		arg_count: u8,
	) -> Result<()> {
		todo!()
	}

	fn call_value(&mut self, callee: Value, arg_count: u8) -> Result<()> {
		let Some(obj) = callee.as_obj() else {
			return Err(eyre!("Can only call functions and classes."))
		};

		if let Some(klass) = obj.try_cast::<ObjClass>() {
			let instance_slot =
				&self.stack[self.stack.len() - (arg_count as usize) - 1];
			todo!()
		} else if let Some(closure) = obj.try_cast::<ObjClosure>() {
			// self.call(closure, arg_count)
			todo!()
		} else if let Some(function) = obj.try_cast::<ObjFunction>() {
			// self.call(function, arg_count)
			todo!()
		} else if let Some(native) = obj.try_cast::<ObjNative>() {
			let arg_count = arg_count as usize;
			let res = (native.function)(unsafe {
				self.stack_slice_from_top(arg_count as _)
			});
			self.stack.pop_n(arg_count);
			self.push(res);
			return Ok(());
		} else {
			Err(eyre!("Can only call functions and classes."))
		}
	}

	fn capture_upvalues(
		&mut self,
		values: NonNull<Value>,
	) -> GcRef<ObjUpvalue> {
		todo!()
	}

	fn binary_op(&mut self, op: impl FnOnce(f64, f64) -> Value) {
		todo!()
	}

	fn close_upvalues(&mut self, last: *const Value) {
		todo!()
	}

	fn frame<'frame, 'vm: 'frame>(&'vm self) -> &'frame CallFrame {
		&self.frames[self.frames.len() - 1]
	}

	fn frame_mut<'frame, 'vm: 'frame>(&'vm mut self) -> &'frame mut CallFrame {
		todo!()
	}

	fn new_instance(&mut self, klass: GcRef<ObjClass>) -> GcRef<ObjInstance> {
		todo!()
	}

	fn read_byte(&mut self) -> Bytecode {
		// let res = self.stack[]
		todo!()
	}

	fn read_constant(&mut self) -> Value {
		todo!()
	}

	fn read_short(&mut self) -> u16 {
		todo!()
	}

	fn read_string(&mut self) -> GcRef<ObjString> {
		todo!()
	}

	unsafe fn stack_slice_from_top<'slice, 'me: 'slice>(
		&'me self,
		len: usize,
	) -> &'slice [Value] {
		todo!()
	}
}
