/*
 * This Java source file was generated by the Gradle 'init' task.
 */
package lox;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.nio.charset.Charset;
import java.nio.file.Files;
import java.nio.file.Paths;

public class Lox {
	private static final Interpreter interpreter = new Interpreter();
	static boolean hadError = false;
	static boolean hadRuntimeError = false;

	public static void main(String[] args) throws IOException {
		if (args.length > 1) {
			System.out.println("Usage: jlox [script]");
			System.exit(64);
		}

		if (args.length == 0) {
			runPrompt();
		} else {
			runFile(args[0]);
		}
	}

	private static void runFile(String path) throws IOException {
		var bytes = Files.readAllBytes(Paths.get(path));
		run(new String(bytes, Charset.defaultCharset()));
		if (hadError)
			System.exit(65);
		if (hadRuntimeError)
			System.exit(70);
	}

	private static void runPrompt() throws IOException {
		var input = new InputStreamReader(System.in);
		var bufferedReader = new BufferedReader(input);

		for (;;) {
			System.out.print("> ");
			var line = bufferedReader.readLine();
			if (line == null)
				break;
			run(line);
			hadError = false;
		}
	}

	private static void run(String input) {
		var scanner = new Scanner(input);
		var tokens = scanner.scanTokens();
		var parser = new Parser(tokens);
		var statements = parser.parse();

		// stop if there was a syntax error
		if (hadError)
			return;

		var resolver = new Resolver(interpreter);
		resolver.resolve(statements);

		interpreter.interpret(statements);
	}

	static void error(int line, String message) {
		report(line, "", message);
	}

	static void error(Token token, String message) {
		if (token.type == TokenType.EOF) {
			report(token.line, " at end", message);
		} else {
			report(token.line, " at '" + token.lexeme + "'", message);
		}
	}

	static void runtimeError(RuntimeError error) {
		System.err.println(error.getMessage());
		System.err.println("[line " + error.token.line + "]");
		hadRuntimeError = true;
	}

	static void report(int line, String where, String message) {
		System.err.println("[line " + line + "] Error" + where + ": " + message);
		hadError = true;
	}
}
