cmake_minimum_required(VERSION 3.23)

project(clox)

add_executable(clox
	clox.c
	common.h

	chunk.h chunk.c
	debug.h debug.c
	memory.h memory.c
	value.h
	)
