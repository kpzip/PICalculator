#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include "pic16machine.h"

int main() {

	PIC16Machine pic;
	memset(&pic, 0, sizeof(PIC16Machine));
	pic.io.PORTB = 0xFF;
	uint8_t *accessed = getRegFile(&pic, 0x06);
	printf("PORTB: %d", *accessed);
	return 0;
}
