/*
 * pic16machine.c
 *
 *  Created on: Jan 13, 2025
 *      Author: kpzip
 */
#include <assert.h>
#include "pic16machine.h"

PIC16Machine::PIC16Machine(size_t eeprom_size)
	: zero(0),
	prgm_memory((uint16_t*)malloc(sizeof(uint16_t) * eeprom_size)),
	prgm_memory_size(eeprom_size)
	{}

PIC16Machine::~PIC16Machine() {
	free(prgm_memory);
}

uint16_t *PIC16Machine::getPrgrmMemory() {
	return this->prgm_memory;
}

size_t PIC16Machine::getPrgrmMemorySize() {
	return this->prgm_memory_size;
}

PIC16Instruction PIC16Machine::decode(uint16_t instruction) {
	return PIC16Instruction(instruction);
}




