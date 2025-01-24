/*
 * C74.h
 *
 *  Created on: Jan 19, 2025
 *      Author: kpzip
 */

#ifndef SRC_C74_H_
#define SRC_C74_H_

#include "pic16machine.h"
#include "pic16instructionset.h"

#define C74_MEM_SIZE 4000

typedef struct {
	// BANK 0
	const uint8_t INDF[1] = { 0 };
	uint8_t TMR0;
	uint8_t PCL;
	uint8_t STATUS;
	uint8_t FSR;
	uint8_t PORTA;
	uint8_t PORTB;
	uint8_t PORTC;
	uint8_t PORTD;
	uint8_t PORTE;
	uint8_t PCLATH;
	uint8_t INTCON;
	uint8_t PIR1;
	uint8_t PIR2;
	uint8_t TMR1L;
	uint8_t TMR1H;
	uint8_t T1CON;
	uint8_t TMR2;
	uint8_t T2CON;
	uint8_t SSPBUF;
	uint8_t SSPCON;
	uint8_t CCPR1L;
	uint8_t CCPR1H;
	uint8_t CCP1CON;
	uint8_t RCSTA;
	uint8_t TXREG;
	uint8_t RCREG;
	uint8_t CCPR2L;
	uint8_t CCPR2H;
	uint8_t CCP2CON;
	uint8_t ADRES;
	uint8_t ADCON0;

	// BANK 1
	const uint8_t PADDING1[1] = { 0 };
	uint8_t OPTION;
	const uint8_t PADDING2[3] = { 0 };
	uint8_t TRISA;
	uint8_t TRISB;
	uint8_t TRISC;
	uint8_t TRISD;
	uint8_t TRISE;
	const uint8_t PADDING3[2] = { 0 };
	uint8_t PIE1;
	uint8_t PIE2;
	uint8_t PCON;
	const uint8_t PADDING4[3] = { 0 };
	uint8_t PR2;
	uint8_t SSPADD;
	uint8_t SSPSTAT;
	const uint8_t PADDING5[3] = { 0 };
	uint8_t TXSTA;
	uint8_t SPBRG;
	const uint8_t PADDING6[5] = { 0 };
	uint8_t ADCON1;
} C74IORegisters;

typedef struct {
	uint8_t bank0[96];
	uint8_t bank1[96];
	uint8_t bank2[96];
	uint8_t bank3[96];
} C74GPRegisters;

typedef struct {
	uint8_t W;
	uint16_t INST;
	uint16_t PC;
} C74CPURegisters;

class PIC16C74 final: PIC16Machine {
private:
	uint16_t stack[8];
	uint8_t SP;
	C74IORegisters io;
	C74GPRegisters gpr;
	C74CPURegisters cpur;
public:
	PIC16C74()
	: PIC16Machine(C74_MEM_SIZE),
	  stack{0},
	  SP(0),
	  io({}),
	  gpr({}),
	  cpur({})
	{}

	C74IORegisters *ioReg() {
		return &this->io;
	}

	virtual uint8_t *getRegFile(uint16_t addr) override;
	virtual uint16_t fetch() override;
	virtual void execute() override;
};


#endif /* SRC_C74_H_ */
