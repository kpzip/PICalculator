#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <stdlib.h>

#include "pic16machine.h"
#include "pic16instructionset.h"

int main() {

	PIC16Machine *pic = new PIC16Machine();

	pic->io.PORTB = 0xAA;
	pic->io.FSR = 0x06;
	pic->io.STATUS |= 0b10000000;
	uint8_t *accessed = pic->getRegFile(0x00);
	printf("PORTB: %d\n", *accessed);
	*accessed += 1;
	printf("PORTB: %d\n", *accessed);
	return 0;
}
