#ifndef clox_vm_h
#define clox_vm_h

#include "chunk.h"

typedef struct {
	Chunk* chunk;
	// better to store in a local variable to keep in a register
	uint8_t* ip;
} VM;

typedef enum {
	INTERPRET_OK,
	INTERPRET_COMPILE_ERROR,
	INTERPRET_RUNTIME_ERROR
} InterpretResult;

void initVM();
void freeVM();
InterpretResult interpret(Chunk* chunk);

#endif // clox_vm_h
