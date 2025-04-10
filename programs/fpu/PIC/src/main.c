// Comment out for testing
#include <16c74.h>

#include "definitions.h"

// Comment these out when compiling for the pic
//#include "test/test_defs.h"
//#include "test/main.h"
//#include <stdio.h>

// Clock settings
// Comment out for testing
// TODO define these since they are different on our pic
#fuses PLL_DIV_1 PLL_DIV_4
#use delay(clock=20M, RESTART_WDT)

#define SPI_DELAY 10

// LED statuses
static uint8_t second = 0;
static uint8_t alpha = 0;
static uint8_t degrees = 0;

void set_keyboard_row(uint8_t row) {
	
	// Not sure how necessary all the bit shifting is since the single bit representation is ambiguous
	output_bit(KEYBOARD_0, row & 0x01);
	output_bit(KEYBOARD_1, (row >> 1) & 0x01);
	output_bit(KEYBOARD_2, (row >> 2) & 0x01); 
	
	//output_a(row);
}

void display_command(uint8_t data, uint8_t rs) {
	output_low(DSS);
	spi_write(0xF8 | (rs << 1));
	spi_write(data & 0xF0);
	spi_write((data << 4) & 0xF0);
	output_high(DSS);

	delay_ms(SPI_DELAY);
}

void send_to_STM(uint8_t b) {
	output_low(COSS);
	// spi_write(0x01); // PIC -> STM
	// delay_ms(SPI_DELAY);
	spi_write(0x01); // Key Pressed
	//delay_ms(SPI_DELAY);
	spi_write(b);    // Key Pressed Data
	output_high(COSS);
}

uint8_t handle_possible_STM_response() {
	output_low(COSS);
	/* spi_write(0x00); // STM -> PIC
	while (TRUE) {
		delay_ms(SPI_DELAY);
		uint8_t status = spi_read(0x00); // Read STM Status
		switch(status) {
		case 0: // Still Waiting
			continue;
		case 1: // No data to recieve
			return 0;
		case 2: // Display Command
			break;
		default:
			return 0;
		}
	} */
	
	while (!input(CO_READY)); // Wait for STM to finish
	
	uint8_t display_rs;
	uint8_t more_data;
	
	do {
		
		uint8_t status = spi_read(0x00); // Read STM status
		
		switch (status & 0x7F) { // Cut off top bit
		case 0: // No data
			return 0;
		case 1: // Display command, RS=0
			display_rs = 0;
			break;
		case 2: // Display command, RS=1
			display_rs = 1;
			break;
		default:
			return 0;
		}
		
		uint8_t display_byte = spi_read(0x00);
	
		output_high(COSS);
		display_command(display_byte, display_rs);
		
		// Top bit is set if the STM has more data to send
		more_data = status & 0x80;
		
	} while (more_data);

	return 1;
}

void main() {
	// Initialize Pins

	set_tris_a(0x00);
	set_tris_b(0xFF);
	set_tris_d(0x00);
	set_tris_e(0x00);
	
	// Set STM ready pin and MISO as inputs
	set_tris_c(0b00010010);

	// Chip Select pins
	// Award winning unimplemented Chip Select!!!
	output_high(DSS);
	output_high(COSS);

	disable_interrupts(GLOBAL);

	// Keyboard matrix pins
	output_low(KEYBOARD_0);
	output_low(KEYBOARD_1);
	output_low(KEYBOARD_2);

	// Indicator LEDs
	output_high(SECOND_LED);
	output_high(ALPHA_LED);
	output_high(RADIANS_LED);
	output_high(DEGREES_LED);

	delay_ms(1000);

	output_low(SECOND_LED);
	output_low(ALPHA_LED);
	output_low(RADIANS_LED);
	output_low(DEGREES_LED);

	// Init the SPI bus
	setup_spi(SPI_MASTER | SPI_SCK_IDLE_HIGH);

	// Initialize display
	display_command(0x30, 0); // Function set: 8 bit interface, basic instruction set
	// display_command(0x08, 0); // Display status: Everything off
	// display_command(0x10, 0); // Cursor: Move left (?)
	display_command(0x0E, 0); // Display status: Display, cursor, and blink on
	display_command(0x01, 0); // Clear
	display_command(0x06, 0); // Make cursor move right
	// display_command(0x80, 0); // Home the cursor (unneccesary)

	// Sync
	output_high(RADIANS_LED);
	
	// Button press event loop
	uint8_t last_button = 0xFF;
	while (TRUE) {
		// Check for input: scan across different button ranks and determine the button pressed if any
		uint8_t any_button_pressed = 0;
		for (uint8_t row_counter = 0; row_counter < 7; row_counter++) {
			set_keyboard_row(row_counter);
			for (column_counter = 0; column_counter < 6; column_counter++) {
				if (input(KEYBOARD_IN + column_counter)) {
					uint8_t button_id = column_counter + row_counter * 6;
					if (last_button != button_id) {
						last_button = button_id;
						any_button_pressed = 1;
					} else {
						any_button_pressed = 2;
					}
				}
			}
		}
		if (any_button_pressed == 2) {
			// Do Nothing since we just re-registered the last button press
		} else if (any_button_pressed == 1) {
			// Send button press info to STM and set LED pins if need be
			send_to_STM(last_button);
			switch(last_button) {
			case 12:
				// Deg/Rad toggle
				if (second) {
					degrees = 0;
					output_low(DEGREES_LED);
					output_high(RADIANS_LED);
				} else {
					degrees = 1;
					output_high(DEGREES_LED);
					output_low(RADIANS_LED);
				}
				break;
			case 24:
				// Alpha
				if (alpha) {
					alpha = 0;
					output_low(ALPHA_LED);
				} else {
					alpha = 1;
					output_high(ALPHA_LED);
				}
				break;
			case 30:
				// 2nd
				if (second) {
					status = 0;
					output_low(SECOND_LED);
				} else {
					status = 1;
					output_high(SECOND_LED);
				}
				break;
			default:
				// Possibly turn off second and alpha here?
				//second = 0;
				//alpha = 0;
				break;
			}
			
			// Get display commands from STM
			//while(handle_possible_STM_response()) {}
			handle_possible_STM_response();
			
		else {
			last_button = 0xFF;
		}	
	}
}
