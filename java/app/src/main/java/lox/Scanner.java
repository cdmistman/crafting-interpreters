package lox;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

import static lox.TokenType.*;

public class Scanner {
	private static final Map<String, TokenType> keywords;

	static {
		keywords = new HashMap<>();
		keywords.put("and", AND);
		keywords.put("class", CLASS);
		keywords.put("else", ELSE);
		keywords.put("false", FALSE);
		keywords.put("for", FOR);
		keywords.put("fun", FUN);
		keywords.put("if", IF);
		keywords.put("nil", NIL);
		keywords.put("or", OR);
		keywords.put("print", PRINT);
		keywords.put("return", RETURN);
		keywords.put("super", SUPER);
		keywords.put("this", THIS);
		keywords.put("true", TRUE);
		keywords.put("var", VAR);
		keywords.put("while", WHILE);
	}

	private final String source;
	private final List<Token> tokens = new ArrayList<>();

	private int start = 0;
	private int current = 0;
	private int line = 1;

	public Scanner(String source) {
		this.source = source;
	}

	public List<Token> scanTokens() {
		while (!this.isAtEnd()) {
			// we are at the beginning of the next lexeme
			this.start = this.current;
			this.scanToken();
		}

		this.tokens.add(new Token(EOF, "", null, this.line));
		return this.tokens;
	}

	private void addToken(TokenType type) {
		this.addToken(type, null);
	}

	private void addToken(TokenType type, Object literal) {
		String text = this.source.substring(this.start, this.current);
		this.tokens.add(new Token(type, text, literal, this.line));
	}

	private char advance() {
		return this.source.charAt(this.current++);
	}

	private void blockComment() {
		for (;;) {
			char c = this.advance();
			switch (c) {
				case '\n':
					this.line++;
					break;

				case '/':
					if (this.match('*')) {
						// recurse into block comment
						this.blockComment();
					}
					break;

				case '*':
					if (this.match('/')) {
						return;
					}

				default:
					break;
			}
		}
	}

	private void identifier() {
		while (isAlphaNumeric(this.peek()))
			this.advance();

		var text = this.source.substring(this.start, this.current);
		var type = keywords.get(text);
		if (type == null) {
			type = IDENTIFIER;
		}

		this.addToken(type);
	}

	private boolean isAtEnd() {
		return this.current >= this.source.length();
	}

	private boolean match(char expected) {
		if (this.isAtEnd())
			return false;
		if (this.source.charAt(this.current) != expected)
			return false;

		this.current++;
		return true;
	}

	private void number() {
		while (isDigit(this.peek()))
			this.advance();

		// look for a fractional part
		if (this.peek() == '.' && isDigit(this.peekNext())) {
			// consume the "."
			this.advance();

			while (isDigit(this.peek()))
				advance();
		}

		this.addToken(NUMBER, Double.parseDouble(this.source.substring(this.start, this.current)));
	}

	private char peek() {
		if (this.isAtEnd())
			return '\0';
		return this.source.charAt(this.current);
	}

	private char peekNext() {
		var next = this.current + 1;
		if (next >= this.source.length())
			return '\0';
		return this.source.charAt(next);
	}

	private void scanToken() {
		char c = this.advance();
		switch (c) {
			case '(':
				this.addToken(LEFT_PAREN);
				break;
			case ')':
				this.addToken(RIGHT_PAREN);
				break;
			case '{':
				this.addToken(LEFT_BRACE);
				break;
			case '}':
				this.addToken(RIGHT_BRACE);
				break;
			case ',':
				this.addToken(COMMA);
				break;
			case '.':
				this.addToken(DOT);
				break;
			case '-':
				this.addToken(MINUS);
				break;
			case '+':
				this.addToken(PLUS);
				break;
			case ';':
				this.addToken(SEMICOLON);
				break;
			case '*':
				this.addToken(STAR);
				break;
			case '!':
				this.addToken(this.match('=') ? BANG_EQUAL : BANG);
				break;
			case '=':
				this.addToken(this.match('=') ? EQUAL_EQUAL : EQUAL);
				break;
			case '<':
				this.addToken(this.match('=') ? LESS_EQUAL : LESS);
				break;
			case '>':
				this.addToken(this.match('=') ? GREATER_EQUAL : GREATER);
				break;
			case '/':
				if (this.match('/')) {
					// a comment goes until the end of a line
					while (this.peek() != '\n' && !this.isAtEnd())
						this.advance();
				} else if (this.match('*')) {
					this.blockComment();
				} else {
					this.addToken(SLASH);
				}
				break;

			case '"':
				this.string();
				break;

			case '\n':
				this.line++;
			case ' ':
			case '\r':
			case '\t':
				// ignore whitespace
				break;

			default:
				if (isDigit(c)) {
					this.number();
				} else if (isAlpha(c)) {
					this.identifier();
				} else {
					Lox.error(this.line, "Unexpected character.");
				}
				break;
		}
	}

	private void string() {
		while (this.peek() != '"' && !this.isAtEnd()) {
			if (this.peek() == '\n')
				this.line++;
			this.advance();
		}

		if (this.isAtEnd()) {
			Lox.error(this.line, "Unterminated string.");
			return;
		}

		// The closing ".
		this.advance();

		// Trim surrounding ""
		var string = source.substring(this.start + 1, this.current - 1);
		this.addToken(STRING, string);
	}

	private static boolean isAlpha(char c) {
		return (c >= 'a' && c <= 'z') ||
				(c >= 'A' && c <= 'Z') ||
				c == '_';
	}

	private static boolean isAlphaNumeric(char c) {
		return isAlpha(c) || isDigit(c);
	}

	private static boolean isDigit(char c) {
		return c >= '0' && c <= '9';
	}
}
