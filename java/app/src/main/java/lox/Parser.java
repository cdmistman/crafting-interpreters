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
			if (match(VAR))
				return varDeclaration();

			return statement();
		} catch (ParseError error) {
			synchronize();
			return null;
		}
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
		if (match(FOR)) return forStatement();
		if (match(IF))
			return ifStatement();
		if (match(PRINT))
			return printStatement();
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

		if (condition == null) condition = new Expr.Literal(true);
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
		return comma();
	}

	private Expr comma() {
		var expr = assignment();

		while (match(COMMA)) {
			var operator = previous();
			var right = comma();
			expr = new Expr.Binary(expr, operator, right);
		}

		return expr;
	}

	private Expr assignment() {
		var expr = ternary();

		if (match(EQUAL)) {
			var equals = previous();
			var value = assignment();

			if (expr instanceof Expr.Variable) {
				var name = ((Expr.Variable) expr).name;
				return new Expr.Assign(name, value);
			}

			error(equals, "Invalid assignment target.");
		}

		return expr;
	}

	private Expr ternary() {
		var cond = or();

		if (!match(QUESTION))
			return cond;

		var question = previous();
		var then = expression();

		if (!match(COLON)) {
			throw error(peek(), "Expect ':' in ternary operation");
		}

		var colon = previous();
		var els = expression();

		var branches = new Expr.Binary(then, colon, els);
		var expr = new Expr.Binary(cond, question, branches);
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

		return primary();
	}

	private Expr primary() {
		if (match(FALSE))
			return new Expr.Literal(false);
		if (match(TRUE))
			return new Expr.Literal(true);
		if (match(NIL))
			return new Expr.Literal(null);

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
