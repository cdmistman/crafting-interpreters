package lox;

import java.util.List;
import java.util.Map;

public class LoxClass implements LoxCallable {
	final String name;
	final LoxClass superclass;
	private final Map<String, LoxFunction> methods;

	LoxClass(String name, LoxClass superclass, Map<String, LoxFunction> methods) {
		this.name = name;
		this.superclass = superclass;
		this.methods = methods;
	}

	LoxFunction findMethod(String name) {
		if (methods.containsKey(name)) {
			return methods.get(name);
		}

		if (superclass != null) {
			return superclass.findMethod(name);
		}

		return null;
	}

	@Override
	public int arity() {
		var initializer = findMethod("init");
		if (initializer == null)
			return 0;
		return initializer.arity();
	}

	@Override
	public Object call(Interpreter interpreter, List<Object> arguments) {
		var instance = new LoxInstance(this);
		var initializer = findMethod("init");
		if (initializer != null) {
			initializer.bind(instance).call(interpreter, arguments);
		}
		return instance;
	}

	@Override
	public String toString() {
		return name;
	}
}
