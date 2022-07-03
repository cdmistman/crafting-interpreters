#include <stdio.h>
#include <stdlib.h>

#include "memory.h"

void* reallocate(void* pointer, size_t oldSize, size_t newSize) {
	if (newSize == 0) {
		free(pointer);
		return NULL;
	}

	void* result;
	if (pointer == NULL || oldSize == 0) {
		if (pointer == NULL ^ oldSize == 0) {
			printf("uh oh");
		}
		result = malloc(newSize);
	} else {
		result = realloc(pointer, newSize);
	}
	if (result == NULL)
		exit(1);
	return result;
}
