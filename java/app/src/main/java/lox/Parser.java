package lox;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

import static lox.TokenType.*;

public class Parser {
	private static class ParseError extends RuntimeException {
	}

	private final List<Token> tokens;
	private int current = 0;

	Parser(List<Token> tokens) {
		this.tokens = tokens;
	}

	List<Stmt> parse() {
		var statements = new ArrayList<Stmt>();
		while (!isAtEnd()) {
			statements.add(declaration());
		}

		return statements;
	}

	private Stmt declaration() {
		try {
			if (match(CLASS))
				return classDeclaration();
			if (match(FUN))
				return function("function");
			if (match(VAR))
				return varDeclaration();

			return statement();
		} catch (ParseError error) {
			synchronize();
			return null;
		}
	}

	private Stmt.Class classDeclaration() {
		var name = consume(IDENTIFIER, "Expect class name.");
		consume(LEFT_BRACE, "Expect '{' before class body.");

		var methods = new ArrayList<Stmt.Function>();
		while (!check(RIGHT_BRACE) && !isAtEnd()) {
			methods.add(function("method"));
		}

		consume(RIGHT_BRACE, "Expect '}' after class body.");

		return new Stmt.Class(name, methods);
	}

	private Stmt.Function function(String kind) {
		var name = consume(IDENTIFIER, "Expect " + kind + " name.");

		// parameter list and parentheses wrapped around it
		consume(LEFT_PAREN, "Expect '(' after " + kind + " name.");
		var parameters = new ArrayList<Token>();
		if (!check(RIGHT_PAREN)) {
			do {
				if (parameters.size() >= 255) {
					error(peek(), "Can't have more than 255 parameters.");
				}
				parameters.add(consume(IDENTIFIER, "Expect parameter name"));
			} while (match(COMMA));
		}
		consume(RIGHT_PAREN, "Expect ')' after parameters.");

		consume(LEFT_BRACE, "Expect '{' before " + kind + " body.");
		var body = block();
		return new Stmt.Function(name, parameters, body);
	}

	private Stmt varDeclaration() {
		var name = consume(IDENTIFIER, "Expect variable name.");

		Expr initializer = null;
		if (match(EQUAL)) {
			initializer = expression();
		}

		consume(SEMICOLON, "Expect ';' after variable declaration");
		return new Stmt.Var(name, initializer);
	}

	private Stmt statement() {
		if (match(LEFT_BRACE))
			return new Stmt.Block(block());
		if (match(FOR))
			return forStatement();
		if (match(IF))
			return ifStatement();
		if (match(PRINT))
			return printStatement();
		if (match(RETURN))
			return returnStatement();
		if (match(WHILE))
			return whileStatement();

		return expressionStatement();
	}

	private List<Stmt> block() {
		var statements = new ArrayList<Stmt>();

		while (!check(RIGHT_BRACE) && !isAtEnd()) {
			statements.add(declaration());
		}

		consume(RIGHT_BRACE, "Expect '}' after block.");
		return statements;
	}

	private Stmt forStatement() {
		consume(LEFT_PAREN, "Expect '(' after 'for'.");

		Stmt initializer;
		if (match(SEMICOLON)) {
			initializer = null;
		} else if (match(VAR)) {
			initializer = varDeclaration();
		} else {
			initializer = expressionStatement();
		}

		Expr condition = null;
		if (!check(SEMICOLON)) {
			condition = expression();
		}
		consume(SEMICOLON, "Expect ';' after loop condition.");

		Expr increment = null;
		if (!check(RIGHT_PAREN)) {
			increment = expression();
		}
		consume(RIGHT_PAREN, "Expect ')' after for clauses.");

		var body = statement();

		if (increment != null) {
			body = new Stmt.Block(
					Arrays.asList(body, new Stmt.Expression(increment)));
		}

		if (condition == null) {
			condition = new Expr.Literal(true);
		}
		body = new Stmt.While(condition, body);

		if (initializer != null) {
			body = new Stmt.Block(Arrays.asList(initializer, body));
		}

		return body;
	}

	private Stmt ifStatement() {
		consume(LEFT_PAREN, "Expect '(' after 'if'.");
		var condition = expression();
		consume(RIGHT_PAREN, "Expect ')' after if condition.");

		var thenBranch = statement();
		Stmt elseBranch = null;
		if (match(ELSE)) {
			elseBranch = statement();
		}

		return new Stmt.If(condition, thenBranch, elseBranch);
	}

	private Stmt printStatement() {
		var value = expression();
		consume(SEMICOLON, "Expect ';' after value.");
		return new Stmt.Print(value);
	}

	private Stmt returnStatement() {
		var keyword = previous();
		Expr value = null;
		if (!check(SEMICOLON)) {
			value = expression();
		}

		consume(SEMICOLON, "Expect ';' after return value.");
		return new Stmt.Return(keyword, value);
	}

	private Stmt whileStatement() {
		consume(LEFT_PAREN, "Expect '(' after 'while'.");
		var condition = expression();
		consume(RIGHT_PAREN, "Expect ')' after condition.");
		var body = statement();

		return new Stmt.While(condition, body);
	}

	private Stmt expressionStatement() {
		var expr = expression();
		consume(SEMICOLON, "Expect ';' after expression.");
		return new Stmt.Expression(expr);
	}

	private Expr expression() {
		return assignment();
	}

	private Expr assignment() {
		var expr = or();

		if (match(EQUAL)) {
			var equals = previous();
			var value = assignment();

			if (expr instanceof Expr.Variable) {
				var name = ((Expr.Variable) expr).name;
				return new Expr.Assign(name, value);
			} else if (expr instanceof Expr.Get) {
				var get = (Expr.Get)expr;
				return new Expr.Set(get.object, get.name, value);
			}

			error(equals, "Invalid assignment target.");
		}

		return expr;
	}

	private Expr or() {
		var expr = and();

		while (match(OR)) {
			var operator = previous();
			var right = and();
			expr = new Expr.Logical(expr, operator, right);
		}

		return expr;
	}

	private Expr and() {
		var expr = equality();

		while (match(AND)) {
			var operator = previous();
			var right = equality();
			expr = new Expr.Logical(expr, operator, right);
		}

		return expr;
	}

	private Expr equality() {
		var expr = comparison();

		while (match(BANG_EQUAL, EQUAL_EQUAL)) {
			var operator = previous();
			var right = comparison();
			expr = new Expr.Binary(expr, operator, right);
		}

		return expr;
	}

	private Expr comparison() {
		var expr = term();

		while (match(GREATER, GREATER_EQUAL, LESS, LESS_EQUAL)) {
			var operator = previous();
			var right = term();
			expr = new Expr.Binary(expr, operator, right);
		}

		return expr;
	}

	private Expr term() {
		var expr = factor();

		while (match(MINUS, PLUS)) {
			var operator = previous();
			var right = factor();
			expr = new Expr.Binary(expr, operator, right);
		}

		return expr;
	}

	private Expr factor() {
		var expr = unary();

		while (match(SLASH, STAR)) {
			var operator = previous();
			var right = unary();
			expr = new Expr.Binary(expr, operator, right);
		}

		return expr;
	}

	private Expr unary() {
		if (match(BANG, MINUS)) {
			var operator = previous();
			var right = unary();
			return new Expr.Unary(operator, right);
		}

		return call();
	}

	private Expr call() {
		var expr = primary();

		while (true) {
			if (match(LEFT_PAREN)) {
				expr = finishCall(expr);
			} else if (match(DOT)) {
				var name = consume(IDENTIFIER, "Expect property name after '.'.");
				expr = new Expr.Get(expr, name);
			} else {
				break;
			}
		}

		return expr;
	}

	private Expr finishCall(Expr callee) {
		var arguments = new ArrayList<Expr>();

		if (!check(RIGHT_PAREN)) {
			do {
				if (arguments.size() >= 255) {
					error(peek(), "Can't have more than 255 arguments.");
				}
				arguments.add(expression());

			} while (match(COMMA));
		}

		var paren = consume(RIGHT_PAREN, "Expect ')' after arguments.");
		return new Expr.Call(callee, paren, arguments);
	}

	private Expr primary() {
		if (match(FALSE))
			return new Expr.Literal(false);
		if (match(TRUE))
			return new Expr.Literal(true);
		if (match(NIL))
			return new Expr.Literal(null);
		if (match(THIS)) return new Expr.This(previous());

		if (match(NUMBER, STRING)) {
			return new Expr.Literal(previous().literal);
		}
		if (match(IDENTIFIER)) {
			return new Expr.Variable(previous());
		}

		if (match(LEFT_PAREN)) {
			var expr = expression();
			consume(RIGHT_PAREN, "Expect ')' after expression.");
			return new Expr.Grouping(expr);
		}

		throw error(peek(), "Expect expression.");
	}

	private boolean match(TokenType... types) {
		for (var type : types) {
			if (check(type)) {
				advance();
				return true;
			}
		}

		return false;
	}

	private Token consume(TokenType type, String message) {
		if (check(type))
			return advance();

		throw error(peek(), message);
	}

	private boolean check(TokenType type) {
		if (isAtEnd())
			return false;
		return peek().type == type;
	}

	private Token advance() {
		if (!isAtEnd())
			current++;
		return previous();
	}

	private boolean isAtEnd() {
		return peek().type == EOF;
	}

	private Token peek() {
		return tokens.get(current);
	}

	private Token previous() {
		return tokens.get(current - 1);
	}

	private ParseError error(Token token, String message) {
		Lox.error(token, message);
		return new ParseError();
	}

	private void synchronize() {
		advance();

		while (!isAtEnd()) {
			if (previous().type == SEMICOLON)
				return;

			switch (peek().type) {
				case CLASS:
				case FUN:
				case VAR:
				case FOR:
				case IF:
				case WHILE:
				case PRINT:
				case RETURN:
					return;

				default:
					break;
			}

			advance();
		}
	}
}
