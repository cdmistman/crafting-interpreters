use std::ops::Neg;

use super::*;
use crate::chunk::Bytecode;
use crate::obj::ObjClosure;
use crate::obj::ObjFunction;
use crate::obj::ObjInstance;

impl<const MAX_FRAMES: usize, const STACK_SIZE: usize>
	Vm<MAX_FRAMES, STACK_SIZE>
{
	fn run(&mut self) -> Result<(), ()> {
		if cfg!(feature = "debug-trace") {
			print!(" ");
			for slot in self.stack.iter() {
				if slot as *const _ >= self.stack_top {
					break;
				}
				print!(" [{}] ", unsafe { slot.assume_init_ref() });
			}
			println!();
			// TODO: disassembleInstruction
		}

		loop {
			use crate::chunk::Instr::*;

			let bytecode = self.read_byte();
			match unsafe { bytecode.instr } {
				Closure => {
					let Some(function) = self.read_constant().as_obj::<ObjFunction>() else {
						// TODO: report error. turns out this isn't handled in the book :)
						return Err(());
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
				Nil => self.push(Value::Nil),
				True => self.push(Value::Bool(true)),

				Call => {
					let arg_count = self.read_byte();
					let arg_count = unsafe { arg_count.index };
					if !self.call_value(self.peek(arg_count as _), arg_count) {
						// TODO: report error
						return Err(());
					}
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
						// TODO: report error
						return Err(());
					};
					self.push(*value)
				},
				SetGlobal => {
					let name = self.read_string();
					let value = self.peek(0);
					if let None = self.globals.insert(name, value) {
						self.globals.remove(&name);
						// TODO: report error
						return Err(());
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
					let Some(instance) = self.peek(0).as_obj::<ObjInstance>() else {
						// TODO: report error
						return Err(());
					};
					let name = self.read_string();

					let Some(value) = instance.fields.get(&name) else {
						// TODO: report error
						return Err(());
					};

					self.pop(); // instance
					self.push(*value);
				},
				CloseUpvalue => self.close_upvalues(
					self.stack_top.map_addr(|addr| addr - 1).cast(),
				),
				GetUpvalue => {
					let slot = self.read_byte();
					let slot = unsafe { slot.index };
					let value =
						self.frame().closure.upvalues[slot as usize].location;
					self.push(unsafe { value.as_ref() }.clone());
				},
				SetProperty => {
					let Some(mut instance) = self.peek(1).as_obj::<ObjInstance>() else {
						// TODO: report error
						return Err(());
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
					if let Some(a) = l.as_obj::<ObjString>()
						&& let Some(b) = r.as_obj::<ObjString>()
					{
						self.push(ObjString::concat(a, b).value());
					} else if let Value::Number(_) = l && let Value::Number(_) = r {
						self.binary_op(|a, b| Value::Number(a + b))
					} else {
						// TODO: report error
						return Err(());
					}
				},
				Divide => self.binary_op(|a, b| Value::Number(a / b)),
				Multiply => self.binary_op(|a, b| Value::Number(a * b)),
				Negate => {
					let Value::Number(val) = self.peek(0) else {
						// TODO: report error
						return Err(());
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
	fn call_value(&mut self, callee: Value, arg_count: u8) -> bool;

	fn capture_upvalues(
		&mut self,
		values: NonNull<Value>,
	) -> ObjRef<ObjUpvalue>;

	fn binary_op(&mut self, op: impl FnOnce(f64, f64) -> Value);

	fn close_upvalues(&mut self, last: *const Value);

	fn frame<'frame, 'vm: 'frame>(&'vm self) -> &'frame CallFrame;

	fn frame_mut<'frame, 'vm: 'frame>(&'vm mut self) -> &'frame mut CallFrame;

	fn read_byte(&mut self) -> Bytecode;

	fn read_constant(&mut self) -> Value;

	fn read_short(&mut self) -> u16;

	fn read_string(&mut self) -> ObjRef<ObjString>;
}

impl<const MAX_FRAMES: usize, const STACK_SIZE: usize> RunUtil
	for Vm<MAX_FRAMES, STACK_SIZE>
{
	fn call_value(&mut self, callee: Value, arg_count: u8) -> bool {
		todo!()
	}

	fn capture_upvalues(
		&mut self,
		values: NonNull<Value>,
	) -> ObjRef<ObjUpvalue> {
		todo!()
	}

	fn binary_op(&mut self, op: impl FnOnce(f64, f64) -> Value) {
		todo!()
	}

	fn close_upvalues(&mut self, last: *const Value) {
		todo!()
	}

	fn frame<'frame, 'vm: 'frame>(&'vm self) -> &'frame CallFrame {
		&self.frames[self.frame_count - 1]
	}

	fn frame_mut<'frame, 'vm: 'frame>(&'vm mut self) -> &'frame mut CallFrame {
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

	fn read_string(&mut self) -> ObjRef<ObjString> {
		todo!()
	}
}
