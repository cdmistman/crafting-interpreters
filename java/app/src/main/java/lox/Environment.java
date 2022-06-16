package lox;

import java.util.HashMap;
import java.util.Map;

public class Environment {
	public final Environment enclosing;
	private final Map<String, Object> values = new HashMap<>();

	Environment() {
		enclosing = null;
	}

	Environment(Environment enclosing) {
		this.enclosing = enclosing;
	}

	public Object get(Token name) {
		if (values.containsKey(name.lexeme)) {
			return values.get(name.lexeme);
		}

		if (enclosing != null)
			return enclosing.get(name);

		throw new RuntimeError(name, "Undefined variable '" + name.lexeme + "'.");
	}

	public void define(String name, Object value) {
		values.put(name, value);
	}

	Environment ancestor(int distance) {
		var environment = this;
		for (var i = 0; i < distance; i++) {
			assert environment != null;
			environment = environment.enclosing;
		}
		assert environment != null;
		return environment;
	}

	Object getAt(int distance, String name) {
		return ancestor(distance).values.get(name);
	}

	public void assign(Token name, Object value) {
		if (values.containsKey(name.lexeme)) {
			values.put(name.lexeme, value);
			return;
		}

		if (enclosing != null) {
			enclosing.assign(name, value);
			return;
		}

		throw new RuntimeError(name, "Undefined variable '" + name.lexeme + "'.");
	}

	public void assignAt(int distance, Token name, Object value) {
		ancestor(distance).values.put(name.lexeme, value);
	}
}
