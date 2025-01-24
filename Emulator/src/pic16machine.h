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
#include "pic16instructionset.h"

class PIC16Machine {
private:
	uint16_t *prgm_memory;
	size_t prgm_memory_size;

protected:
	uint8_t zero;

public:
	virtual ~PIC16Machine();

	uint16_t *getPrgrmMemory();
	size_t getPrgrmMemorySize();

	virtual uint8_t *getRegFile(uint16_t addr) = 0;
	virtual uint16_t fetch() = 0;
	virtual PIC16Instruction decode(uint16_t instruction);
	virtual void execute() = 0;

	PIC16Machine(size_t eeprom_size);

};

#endif /* SRC_PIC16MACHINE_H_ */
