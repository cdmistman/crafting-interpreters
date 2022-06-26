cmake_minimum_required(VERSION 3.23)

project(clox)

add_executable(clox
	main.c
	common.h

	chunk.h chunk.c
	debug.h debug.c
	memory.h memory.c
	value.h value.c
	vm.h vm.c
	)
