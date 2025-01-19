/*
 * pic16instructionset.cpp
 *
 *  Created on: Jan 18, 2025
 *      Author: kpzip
 */
#include "pic16instructionset.h"

PIC16InstructionType fromString(std::string_view str) {
	// TODO
	return NOP;
}

PIC16InstructionType fromOpcode(uint16_t opcode) {
	// TODO
	return NOP;
}

PIC16Instruction::PIC16Instruction(uint16_t opcode)
	: type(fromOpcode(opcode)),
	file_address(0b1111111 & opcode),
	bit_address(0b111 & (opcode >> 7)),
	literal(opcode), // TODO fix for CALL and GOTO instructions
	dest_select(0b1 & (opcode >> 7))
	{}




