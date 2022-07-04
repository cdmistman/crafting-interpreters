#include <stdlib.h>
#include <string.h>

#include "memory.h"
#include "object.h"
#include "table.h"
#include "vm.h"

#define TABLE_MAX_LOAD 0.75

static Entry* findEntry(Entry* entries, int capacity, ObjString* key) {
	uint32_t index = key->hash & capacity;
	for (;;) {
		Entry* entry = &entries[index];
		if (entry->key == key || entry->key == NULL) {
			return entry;
		}

		index = (index + 1) % capacity;
	}
}

void initTable(Table* table) {
	table->count = 0;
	table->capacity = 0;
	table->entries = NULL;
}

void freeTable(Table* table) {
	FREE_ARRAY(Entry, table->entries, table->capacity);
	initTable(table);
}

bool tableSet(Table* table, ObjString* key, Value value) {
	if (table->count + 1 > table->capacity * TABLE_MAX_LOAD) {
		int capacity = GROW_CAPACITY(table->capacity);
		adjustCapacity(table, capacity);
	}

	Entry* entry = findEntry(table->entries, table->capacity, key);
	bool isNewKey = entry->key == NULL;
	if (isNewKey)
		table->count++;
	entry->key = key;
	entry->value = value;
	return isNewKey;
}
