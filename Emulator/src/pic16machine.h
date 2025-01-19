/*
 * pic16machine.h
 *
 *  Created on: Jan 13, 2025
 *      Author: kpzip
 */
#include <stdint.h>

#ifndef SRC_PIC16MACHINE_H_
#define SRC_PIC16MACHINE_H_

typedef struct {
	// BANK 0
	const uint8_t INDF[1];
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
	const uint8_t PADDING1[1];
	uint8_t OPTION;
	const uint8_t PADDING2[3];
	uint8_t TRISA;
	uint8_t TRISB;
	uint8_t TRISC;
	uint8_t TRISD;
	uint8_t TRISE;
	const uint8_t PADDING3[2];
	uint8_t PIE1;
	uint8_t PIE2;
	uint8_t PCON;
	const uint8_t PADDING4[3];
	uint8_t PR2;
	uint8_t SSPADD;
	uint8_t SSPSTAT;
	const uint8_t PADDING5[3];
	uint8_t TXSTA;
	uint8_t SPBRG;
	const uint8_t PADDING6[5];
	uint8_t ADCON1;
} IORegisters;

typedef struct {
	uint8_t bank0[96];
	uint8_t bank1[96];
	uint8_t bank2[96];
	uint8_t bank3[96];
} GPRegisters;

typedef struct {
	uint8_t W;
	uint16_t INST;
	uint16_t PC;
} CPURegisters;


typedef struct {
	uint16_t stack[8];
	uint8_t SP;
	IORegisters io;
	GPRegisters gpr;
	CPURegisters cpur;

} PIC16Machine;

uint8_t *getRegFile(PIC16Machine* machine, uint16_t addr);

#endif /* SRC_PIC16MACHINE_H_ */
