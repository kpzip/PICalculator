/*
 * pic16machine.c
 *
 *  Created on: Jan 13, 2025
 *      Author: kpzip
 */
#include <assert.h>
#include "pic16machine.h"

// addr Low 9 bits are used
uint8_t *PIC16C74::getRegFile(uint16_t addr) {

	static uint8_t ZERO = 0;
	PIC16C74 *machine = this;

	assert(addr <= 0x1FF && "invalid file address");
	C74GPRegisters* gpr = &machine->gpr;
	C74IORegisters* io = &machine->io;
	//uint8_t *gpr_ptr = (uint8_t*)gpr;
	uint8_t *io_ptr = (uint8_t*)io;
	uint8_t bank = addr >> 7;
	uint8_t offset = addr & 0b01111111;
	if (offset == 0) {
		uint16_t new_addr = io->FSR;
		if ((new_addr & 0b01111111) == 0) {
			return &machine->zero;
		}
		new_addr |= ((uint16_t)(io->STATUS & (1 << 7))) << 1;
		return machine->getRegFile(new_addr);
	} else if (bank == 0) {
		if (offset < 0x20) {
			return io_ptr + offset;
		} else {
			return gpr->bank0 + (offset - 0x20);
		}
	} else if (bank == 1) {
		// Check for registers that overlap with bank 0
		if (offset == 0x00 ||
			offset == 0x02 ||
			offset == 0x03 ||
			offset == 0x04 ||
			offset == 0x0A ||
			offset == 0x0B) {
			return io_ptr + offset;
		} else if (offset < 0x20) {
			return io_ptr + offset + 32;
		} else {
			return gpr->bank1 + (offset - 0x20);
		}
	} else if (bank == 2) {
		if (offset <= 0x0F) {
			if (offset == 0x05 ||
				offset == 0x07 ||
				offset == 0x08 ||
				offset == 0x09 ||
				offset == 0x0C ||
				offset == 0x0D ||
				offset == 0x0E ||
				offset == 0x0F) {
				return &machine->zero;
			}
			return io_ptr + offset;
		} else if (offset <= 0x6F) {
			return gpr->bank2 + (offset - 0x10);
		} else {
			return gpr->bank0 + offset;
		}
	} else if (bank == 3) {
		if (offset <= 0x0F) {
			if (offset == 0x05 ||
				offset == 0x07 ||
				offset == 0x08 ||
				offset == 0x09 ||
				offset == 0x0C ||
				offset == 0x0D ||
				offset == 0x0E ||
				offset == 0x0F) {
				return &machine->zero;
			}
			return io_ptr + offset + 32;
		} else if (offset <= 0x6F) {
			return gpr->bank3 + (offset - 0x10);
		} else {
			return gpr->bank0 + offset;
		}
	}
	return &machine->zero;
}




