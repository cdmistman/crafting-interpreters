package lox;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class Interpreter implements Expr.Visitor<Object>, Stmt.Visitor<Void> {
	final Environment globals = new Environment();
	private Environment environment = globals;
	private final Map<Expr, Integer> locals = new HashMap<>();

	Interpreter() {
		globals.define("clock", new LoxCallable() {
			@Override
			public int arity() {
				return 0;
			}

			@Override
			public Object call(Interpreter interpreter, List<Object> arguments) {
				return (double) System.currentTimeMillis() / 1000.0;
			}

			@Override
			public String toString() {
				return "<native fun>";
			}
		});
	}

	void interpret(List<Stmt> statements) {
		try {
			for (var statement : statements) {
				execute(statement);
			}
		} catch (RuntimeError error) {
			Lox.runtimeError(error);
		}
	}

	private String stringify(Object object) {
		if (object == null)
			return "nil";

		if (object instanceof Double) {
			var text = object.toString();
			if (text.endsWith(".0")) {
				text = text.substring(0, text.length() - 2);
			}
			return text;
		}

		return object.toString();
	}

	private void execute(Stmt stmt) {
		stmt.accept(this);
	}

	void resolve(Expr expr, int depth) {
		locals.put(expr, depth);
	}

	void executeBlock(List<Stmt> statements, Environment environment) {
		var previous = this.environment;
		try {
			this.environment = environment;

			for (var statement : statements) {
				execute(statement);
			}
		} finally {
			this.environment = previous;
		}
	}

	private Object evaluate(Expr expr) {
		return expr.accept(this);
	}

	@Override
	public Void visitBlockStmt(Stmt.Block stmt) {
		executeBlock(stmt.statements, new Environment(environment));
		return null;
	}

	@Override
	public Void visitExpressionStmt(Stmt.Expression stmt) {
		evaluate(stmt.expression);
		return null;
	}

	@Override
	public Void visitFunctionStmt(Stmt.Function stmt) {
		var function = new LoxFunction(stmt, environment);
		environment.define(stmt.name.lexeme, function);
		return null;
	}

	@Override
	public Void visitIfStmt(Stmt.If stmt) {
		if (isTruthy(evaluate(stmt.condition))) {
			execute(stmt.thenBranch);
		} else if (stmt.elseBranch != null) {
			execute(stmt.elseBranch);
		}
		return null;
	}

	@Override
	public Void visitPrintStmt(Stmt.Print stmt) {
		var value = evaluate(stmt.expression);
		System.out.println(stringify(value));
		return null;
	}

	@Override
	public Void visitReturnStmt(Stmt.Return stmt) {
		Object value = null;
		if (stmt.value != null)
			value = evaluate(stmt.value);

		throw new Return(value);
	}

	@Override
	public Void visitVarStmt(Stmt.Var stmt) {
		Object value = null;
		if (stmt.initializer != null) {
			value = evaluate(stmt.initializer);
		}

		environment.define(stmt.name.lexeme, value);
		return null;
	}

	@Override
	public Void visitWhileStmt(Stmt.While stmt) {
		while (isTruthy(evaluate(stmt.condition))) {
			execute(stmt.body);
		}
		return null;
	}

	@Override
	public Object visitAssignExpr(Expr.Assign expr) {
		var value = evaluate(expr.value);
		var distance = locals.get(expr);
		if (distance != null) {
			environment.assignAt(distance, expr.name, value);
		} else {
			globals.assign(expr.name, value);
		}

		return value;
	}

	@Override
	public Object visitBinaryExpr(Expr.Binary expr) {
		var left = evaluate(expr.left);

		if (expr.operator.type == TokenType.QUESTION) {
			var branches = (Expr.Binary) expr.right;
			assert branches.operator.type == TokenType.COLON;
			return evaluate(isTruthy(left) ? branches.left : branches.right);
		}

		var right = evaluate(expr.right);

		switch (expr.operator.type) {
			case BANG_EQUAL:
				return !isEqual(left, right);
			case EQUAL_EQUAL:
				return isEqual(left, right);

			case GREATER:
				checkNumberOperands(expr.operator, left, right);
				return (double) left > (double) right;
			case GREATER_EQUAL:
				checkNumberOperands(expr.operator, left, right);
				return (double) left >= (double) right;
			case LESS:
				checkNumberOperands(expr.operator, left, right);
				return (double) left < (double) right;
			case LESS_EQUAL:
				checkNumberOperands(expr.operator, left, right);
				return (double) left <= (double) right;

			case MINUS:
				checkNumberOperands(expr.operator, left, right);
				return (double) left - (double) right;
			case PLUS:
				if (left instanceof Double && right instanceof Double) {
					return (double) left + (double) right;
				} else if (left instanceof String && right instanceof String) {
					return (String) left + (String) right;
				} else {
					throw new RuntimeError(expr.operator, "Operands must be two numbers or two strings.");
				}
			case SLASH:
				checkNumberOperands(expr.operator, left, right);
				return (double) left / (double) right;
			case STAR:
				checkNumberOperands(expr.operator, left, right);
				return (double) left * (double) right;

			default:
				break;
		}

		// unreachable
		return null;
	}

	@Override
	public Object visitCallExpr(Expr.Call expr) {
		var callee = evaluate(expr.callee);

		var arguments = new ArrayList<Object>();
		for (var argument : expr.arguments) {
			arguments.add(evaluate(argument));
		}

		if (!(callee instanceof LoxCallable)) {
			throw new RuntimeError(expr.paren, "Can onlly call functions and classes.");
		}
		var function = (LoxCallable) callee;
		if (arguments.size() != function.arity()) {
			throw new RuntimeError(expr.paren, "Expected " +
					function.arity() + " arguments but got " +
					arguments.size() + ".");
		}
		return function.call(this, arguments);
	}

	@Override
	public Object visitLiteralExpr(Expr.Literal expr) {
		return expr.value;
	}

	@Override
	public Object visitGroupingExpr(Expr.Grouping expr) {
		return evaluate(expr.expression);
	}

	@Override
	public Object visitLogicalExpr(Expr.Logical expr) {
		var left = evaluate(expr.left);

		if (expr.operator.type == TokenType.OR) {
			if (isTruthy(left))
				return left;
		} else {
			if (!isTruthy(left))
				return left;
		}

		return evaluate(expr.right);
	}

	@Override
	public Object visitUnaryExpr(Expr.Unary expr) {
		Object right = evaluate(expr.right);

		switch (expr.operator.type) {
			case BANG:
				return !isTruthy(right);
			case MINUS:
				checkNumberOperand(expr.operator, right);
				return -(double) right;

			default:
				break;
		}

		// unreachable
		return null;
	}

	@Override
	public Object visitVariableExpr(Expr.Variable expr) {
		return lookUpVariable(expr.name, expr);
	}

	private Object lookUpVariable(Token name, Expr expr) {
		var distance = locals.get(expr);
		if (distance != null) {
			return environment.getAt(distance, name.lexeme);
		} else {
			return globals.get(name);
		}
	}

	private void checkNumberOperand(Token operator, Object operand) {
		if (operand instanceof Double)
			return;
		throw new RuntimeError(operator, "Operand must be a number.");
	}

	private void checkNumberOperands(Token operator, Object left, Object right) {
		if (left instanceof Double && right instanceof Double)
			return;
		throw new RuntimeError(operator, "Operands must be numbers.");
	}

	private boolean isEqual(Object a, Object b) {
		if (a == null && b == null)
			return true;
		if (a == null)
			return false;

		return a.equals(b);
	}

	private boolean isTruthy(Object object) {
		if (object == null)
			return false;
		if (object instanceof Boolean)
			return (boolean) object;
		return true;
	}
}
