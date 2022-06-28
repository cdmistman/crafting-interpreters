cmake_minimum_required(VERSION 3.23)

project(clox)

add_executable(clox
	chunk.c chunk.h
	common.h
	compiler.c compiler.h
	debug.c debug.h
	main.c
	memory.c memory.h
	scanner.c scanner.h
	value.c value.h
	vm.c vm.h
	)
