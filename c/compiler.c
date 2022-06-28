#include <stdio.h>

#include "common.h"
#include "compiler.h"
#include "scanner.h"

struct {
	Token current;
	Token previous;
} parser;

static void advance() {
	parser.previous = parser.current;

	for (;;) {
		parser.current = scanToken();
		if (parser.current.type != TOKEN_ERROR)
			break;

		errorAtCurrent(parser.current.start);
	}
}

bool compile(const char* source, Chunk* chunk) {
	initScanner(source);
	advance();
	expression();
	consume(TOKEN_EOF, "Expect end of expression.");
