/*
 * pic16instructionset.cpp
 *
 *  Created on: Jan 18, 2025
 *      Author: kpzip
 */
#include "pic16instructionset.h"

#define INSTR_MATCH(opcode, opcode_base, bitmask, instruction_type) \
	if ((opcode & (~bitmask)) == opcode_base) {\
		return instruction_type;\
	}

PIC16InstructionType fromString(std::string_view str) {
	// TODO
	return NOP;
}

PIC16InstructionType fromOpcode(uint16_t opcode) {

	INSTR_MATCH(opcode, 0b00'0000'0000'0000, 0b00'0000'0110'0000, NOP);

	INSTR_MATCH(opcode, 0b00'0111'0000'0000, 0b00'0000'1111'1111, ADDWF);
	INSTR_MATCH(opcode, 0b00'0101'0000'0000, 0b00'0000'1111'1111, ANDWF);
	INSTR_MATCH(opcode, 0b00'0001'1000'0000, 0b00'0000'0111'1111, CLRF);
	INSTR_MATCH(opcode, 0b00'0001'0000'0000, 0b00'0000'0111'1111, CLRW);
	INSTR_MATCH(opcode, 0b00'1001'0000'0000, 0b00'0000'1111'1111, COMF);
	INSTR_MATCH(opcode, 0b00'0011'0000'0000, 0b00'0000'1111'1111, DECF);
	INSTR_MATCH(opcode, 0b00'1011'0000'0000, 0b00'0000'1111'1111, DECFSZ);
	INSTR_MATCH(opcode, 0b00'1010'0000'0000, 0b00'0000'1111'1111, INCF);
	INSTR_MATCH(opcode, 0b00'1111'0000'0000, 0b00'0000'1111'1111, INCFSZ);
	INSTR_MATCH(opcode, 0b00'0100'0000'0000, 0b00'0000'1111'1111, IORWF);
	INSTR_MATCH(opcode, 0b00'1000'0000'0000, 0b00'0000'1111'1111, MOVF);
	INSTR_MATCH(opcode, 0b00'0000'1000'0000, 0b00'0000'0111'1111, MOVWF);
	INSTR_MATCH(opcode, 0b00'1101'0000'0000, 0b00'0000'1111'1111, RLF);
	INSTR_MATCH(opcode, 0b00'1100'0000'0000, 0b00'0000'1111'1111, RRF);
	INSTR_MATCH(opcode, 0b00'0010'0000'0000, 0b00'0000'1111'1111, SUBWF);
	INSTR_MATCH(opcode, 0b00'1110'0000'0000, 0b00'0000'1111'1111, SWAPF);
	INSTR_MATCH(opcode, 0b00'0110'0000'0000, 0b00'0000'1111'1111, XORWF);

	INSTR_MATCH(opcode, 0b01'0000'0000'0000, 0b00'0011'1111'1111, BCF);
	INSTR_MATCH(opcode, 0b01'0100'0000'0000, 0b00'0011'1111'1111, BSF);
	INSTR_MATCH(opcode, 0b01'1000'0000'0000, 0b00'0011'1111'1111, BTFSC);
	INSTR_MATCH(opcode, 0b01'1100'0000'0000, 0b00'0011'1111'1111, BTFSS);

	INSTR_MATCH(opcode, 0b11'1110'0000'0000, 0b00'0001'1111'1111, ADDLW);
	INSTR_MATCH(opcode, 0b11'1001'0000'0000, 0b00'0000'1111'1111, ANDLW);
	INSTR_MATCH(opcode, 0b10'0000'0000'0000, 0b00'0111'1111'1111, CALL);
	INSTR_MATCH(opcode, 0b00'0000'0110'0100, 0b00'0000'0000'0000, CLRWDT);
	INSTR_MATCH(opcode, 0b10'1000'0000'0000, 0b00'0111'1111'1111, GOTO);
	INSTR_MATCH(opcode, 0b11'1000'0000'0000, 0b00'0000'1111'1111, IORLW);
	INSTR_MATCH(opcode, 0b11'0000'0000'0000, 0b00'0011'1111'1111, MOVLW);
	INSTR_MATCH(opcode, 0b00'0000'0000'1001, 0b00'0000'0000'0000, RETFIE);
	INSTR_MATCH(opcode, 0b11'0100'0000'0000, 0b00'0011'1111'1111, RETLW);
	INSTR_MATCH(opcode, 0b00'0000'0000'0000, 0b00'0000'0000'0000, RETURN);
	INSTR_MATCH(opcode, 0b00'0000'0000'1000, 0b00'0000'0000'0000, SLEEP);
	INSTR_MATCH(opcode, 0b11'1100'0110'0011, 0b00'0001'1111'1111, SUBLW);
	INSTR_MATCH(opcode, 0b11'1010'0000'0000, 0b00'0000'1111'1111, XORLW);

	return NOP;
}

PIC16Instruction::PIC16Instruction(uint16_t opcode)
	: type(fromOpcode(opcode)),
	file_address(0b1111111 & opcode),
	bit_address(0b111 & (opcode >> 7)),
	literal(opcode), // TODO fix for CALL and GOTO instructions
	dest_select(0b1 & (opcode >> 7))
	{}




