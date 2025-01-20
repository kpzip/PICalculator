/*
 * pic16machine.h
 *
 *  Created on: Jan 13, 2025
 *      Author: kpzip
 */

#ifndef SRC_PIC16MACHINE_H_
#define SRC_PIC16MACHINE_H_

#include <stdint.h>
#include <string>

class PIC16Machine {
public:
	uint8_t zero;
	uint16_t *prgm_memory;
	size_t prgm_memory_size;

	virtual uint8_t *getRegFile(uint16_t addr) = 0;
	virtual ~PIC16Machine() {
		free(prgm_memory);
	}

	PIC16Machine(size_t eeprom_size)
	: zero(0),
	prgm_memory((uint16_t*)malloc(sizeof(uint16_t) * eeprom_size)),
	prgm_memory_size(eeprom_size)
	{}

};





#endif /* SRC_PIC16MACHINE_H_ */
