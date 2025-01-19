/*
 * pic16instructionset.h
 *
 *  Created on: Jan 18, 2025
 *      Author: kpzip
 */

#ifndef SRC_PIC16INSTRUCTIONSET_H_
#define SRC_PIC16INSTRUCTIONSET_H_

#include <string_view>
#include <stdint.h>

typedef enum {
	// Byte oriented file register operations
	ADDWF,
	ANDWF,
	CLRF,
	CLRW,
	COMF,
	DECF,
	DECFSZ,
	INCF,
	INCFSZ,
	IORWF,
	MOVF,
	MOVWF,
	NOP,
	RLF,
	RRF,
	SUBWF,
	SWAPF,
	XORWF,

	// Bit Oriented file register operations
	BCF,
	BSF,
	BTFSC,
	BTFSS,

	// Literal & control operations
	ADDLW,
	ANDLW,
	CALL,
	CLRWDT,
	GOTO,
	IORLW,
	MOVLW,
	RETFIE,
	RETLW,
	RETURN,
	SLEEP,
	SUBLW,
	XORLW,

} PIC16InstructionType;

class PIC16Instruction {
private:
	PIC16InstructionType type;
	uint8_t file_address;
	uint8_t bit_address;
	uint8_t literal;
	bool dest_select;
public:
	PIC16Instruction(PIC16InstructionType t)
	: type(t),
	  file_address(0),
	  bit_address(0),
	  literal(0),
	  dest_select(false)
	{}

	static PIC16InstructionType fromString(std::string_view str) const;

};




#endif /* SRC_PIC16INSTRUCTIONSET_H_ */
