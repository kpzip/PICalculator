/*
 * test_defs.c
 *
 *  Created on: Mar 2, 2025
 *      Author: kpzip
 */

#include <stdio.h>

#include "test_defs.h"

void set_tris_a(uint8_t tris) {
	printf("Set tri-state A to: %d\n", tris);
}
void set_tris_b(uint8_t tris) {
	printf("Set tri-state B to: %d\n", tris);
}
void set_tris_c(uint8_t tris) {
	printf("Set tri-state C to: %d\n", tris);
}
void set_tris_d(uint8_t tris) {
	printf("Set tri-state D to: %d\n", tris);
}
void set_tris_e(uint8_t tris) {
	printf("Set tri-state E to: %d\n", tris);
}

void output_high(uint16_t pin) {
	printf("Output set to high on pin: %d\n", pin);
}
void output_low(uint16_t pin) {
	printf("Output set to low on pin: %d\n", pin);
}
void output_bit(uint16_t pin, uint8_t bit) {
	printf("Output set to %d on pin: %d\n", bit, pin);
}

void disable_interrupts(uint16_t) {
	printf("Interrupts Disabled.");
}

void delay_ms(uint16_t delay) {
	printf("Delaying for %d ms.\n", delay);
}

void setup_spi(uint32_t) {
	printf("Spi Initialized.\n");
}
void spi_write(uint8_t val) {
	printf("Wrote %d to spi interface.\n", val);
}
uint8_t spi_read(uint8_t val) {
	printf("Read 0 from spi interface.\n");
	return 0;
}

uint8_t input(uint16_t pin) {
	printf("Read 0 as input on pin:%d\n", pin);
	return 0;
}



