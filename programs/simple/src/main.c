#include <16c74.h>

#include "definitions.h"

// Clock settings
#use delay(clock=20M, RESTARD_WDT)



// Calculator global state
// Bit Vector: each 6 bits represents the y position for each of the 128 x positions.
static uint8_t graphdata[96] = { 0 };

static uint8_t row_counter = 0;
static uint8_t column_counter = 0;
static uint8_t last_button = 255;

// Bit 0: Radians = 0, Degrees = 1; Bit 1: Second mode, 0 = off, Bit2: Alpha mode, 0 = off
static uint8_t status = 0;

static char expr_buffer[20] = { 0 };
static uint8_t write_pointer = 0;


void set_keyboard_row(uint8_t row) {
	// Not sure how necessary all the bit shifting is since the single bit representation is ambiguous
	output_bit(KEYBOARD_0, row & (~0x01));
	output_bit(KEYBOARD_1, (row & (~(0x01 << 1))) >> 1);
	output_bit(KEYBOARD_2, (row & (~(0x01 << 2))) >> 2);
}

void display_command(uint8_t data, uint8_t rs) {
	output_high(DSS);
	spi_write(0xF8 | (rs << 1));
	spi_write(data & 0xF0);
	spi_write((data << 4) & 0xF0);
	output_low(DSS);

	delay_ms(50);
}

void main() {
	// Initialize Pins

	set_tris_a(0x00);
	set_tris_d(0x00);
	set_tris_e(0x00);

	// Chip Select pins
	// Award winning active high chip select for DSS
	output_low(DSS);
	output_high(COSS);

	// Keyboard matrix pins
	output_low(KEYBOARD_0);
	output_low(KEYBOARD_1);
	output_low(KEYBOARD_2);

	// Indicator LEDs
	output_low(SECOND_LED);
	output_low(ALPHA_LED);
	output_low(RADIANS_LED);
	output_low(DEGREES_LED);

	// initialize display
	display_command(0x30, 0); // Function set: 8 bit interface, basic instruction set
	//write_spi(0x08, 0); // Display status: Everything off
	//write_spi(0x10, 0); // Cursor: Move left (?)
	display_command(0x0F, 0); // Display status: Display, cursor, and blink on
	display_command(0x01, 0); // Clear
	display_command(0x06, 0); // Make cursor move right
	//write_spi(0x80, 0); // Home the cursor (unneccesary)

	// Main calculator loop
	while (1) {
		// Check for input: scan across different button ranks and determine the button pressed if any
		uint8_t any_button_pressed = 0;
		for (row_counter = 0; row_counter < 7; row_counter++) {
			set_keyboard_row(row_counter);
			for (column_counter = 0; column_counter < 6; column_counter++) {
				// TODO look at the CCS compiler and figure out if having all global statics is a good idea
				uint8_t is_on = input(KEYBOARD_IN + column_counter);
				if (is_on) {
					any_button_pressed = 1;
					uint8_t button_id = column_counter + row_counter * 6;
					if (last_button != button_id) {
						last_button = button_id;
					}
					else {
						last_button = 255;
					}
				}
			}
		}
		if (any_button_pressed) {
			switch(last_button) {
			case 0:
				write_pointer = 0;
				break;
			case 4:
				// Numpad 0
				expr_buffer[write_pointer++] = '0';
				break;
			case 8:
				// Numpad 7
				expr_buffer[write_pointer++] = '7';
				break;
			case 9:
				// Numpad 8
				expr_buffer[write_pointer++] = '8';
				break;
			case 10:
				// Numpad 9
				expr_buffer[write_pointer++] = '9';
				break;
			case 14:
				// Numpad 4
				expr_buffer[write_pointer++] = '4';
				break;
			case 15:
				// Numpad 5
				expr_buffer[write_pointer++] = '5';
				break;
			case 16:
				// Numpad 6
				expr_buffer[write_pointer++] = '6';
				break;
			case 20:
				// Numpad 1
				expr_buffer[write_pointer++] = '1';
				break;
			case 21:
				// Numpad 2
				expr_buffer[write_pointer++] = '2';
				break;
			case 22:
				// Numpad 3
				expr_buffer[write_pointer++] = '3';
				break;
			default:
				// TODO other cases
				break;
			}

			display_command(0x01, 0);
			for (uint8_t i = 0; i < write_pointer; i++) {
				display_command(expr_buffer[i], 1);
			}


		} else {
			last_button = 255;
		}
	}
}
