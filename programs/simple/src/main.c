#include <16c74.h>

#include "definitions.h"

// Clock settings
#fuses PLL_DIV_1 PLL_DIV_4
#use delay(clock=20M, RESTART_WDT)


// Calculator global state
// Bit Vector: each 6 bits represents the y position for each of the 128 x positions.
//static uint8_t graphdata[96] = { 0 };

static uint8_t row_counter = 0;
static uint8_t column_counter = 0;
static uint8_t last_button = 255;

// Bit 0: Radians = 0, Degrees = 1; Bit 1: Second mode, 0 = off, Bit2: Alpha mode, 0 = off
static uint8_t status = 0;

static char expr_buffer[EXPR_BUF_LEN] = { 0 };
static uint8_t write_pointer = 0;


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

	delay_ms(10);
}

void write_last_character() {
  if (write_pointer != 0) {
		display_command(expr_buffer[write_pointer - 1], 1);
	}
		
	if (write_pointer >= EXPR_BUF_LEN) {
		write_pointer = 0;
	}
}

void main() {
	// Initialize Pins

	set_tris_a(0x00);
	set_tris_b(0xFF);
	set_tris_d(0x00);
	set_tris_e(0x00);

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

	// Main calculator loop
	while (TRUE) {
		// Check for input: scan across different button ranks and determine the button pressed if any
		uint8_t any_button_pressed = 0;
		for (row_counter = 0; row_counter < 7; row_counter++) {
			set_keyboard_row(row_counter);
			for (column_counter = 0; column_counter < 6; column_counter++) {
				// TODO look at the CCS compiler and figure out if having all global statics is a good idea
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
			uint8_t second = (status >> 1) & 1;
			uint8_t alpha = (status >> 2) & 1;
      // Cases are responsible for updating the display
			switch(last_button) {
			case 0:
				write_pointer = 0;
				display_command(0x01, 0); // Clear display
				break;
			case 31:
				output_low(COSS);
				spi_write(status);
				status = spi_read();
				second = (status >> 1) & 1;
				alpha = (status >> 2) & 1;
				uint8_t degrad = status & 1;
				output_bit(SECOND_LED, second);
				output_bit(ALPHA_LED, alpha);
				output_bit(RADIANS_LED, ~degrad);
				output_bit(DEGREES_LED, degrad);
				output_high(COSS);
				break;
			////////////
			// Numpad //
			////////////
			case 3:
				// Numpad 0
				expr_buffer[write_pointer++] = '0';
				write_last_character();
        break;
			case 8:
				// Numpad 7
				expr_buffer[write_pointer++] = '7';
				write_last_character();
        break;
			case 9:
				// Numpad 8
				expr_buffer[write_pointer++] = '8';
				write_last_character();
        break;
			case 10:
				// Numpad 9
				expr_buffer[write_pointer++] = '9';
				write_last_character();
        break;
			case 14:
				// Numpad 4
				expr_buffer[write_pointer++] = '4';
				write_last_character();
        break;
			case 15:
				// Numpad 5
				expr_buffer[write_pointer++] = '5';
        write_last_character();
				break;
			case 16:
				// Numpad 6
				expr_buffer[write_pointer++] = '6';
        write_last_character();
				break;
			case 20:
				// Numpad 1
				expr_buffer[write_pointer++] = '1';
				write_last_character();
        break;
			case 21:
				// Numpad 2
				expr_buffer[write_pointer++] = '2';
				write_last_character();
        break;
			case 22:
				// Numpad 3
				expr_buffer[write_pointer++] = '3';
				write_last_character();
        break;
			/////////////////////////
			// 2nd, Alpha, Deg/Rad //
			/////////////////////////
			case 12:
				// Deg/Rad toggle
				if (second == 0) {
					status |= 1;
					output_high(DEGREES_LED);
					output_low(RADIANS_LED);
				} else {
					status &= (~1);
					output_low(DEGREES_LED);
					output_high(RADIANS_LED);
				}
				break;
			case 24:
				// Alpha
				if (alpha == 0) {
					status |= (1 << 2);
					output_high(ALPHA_LED);
				} else {
					status &= (~(1 << 2));
					output_low(ALPHA_LED);
				}
				break;
			case 30:
				// 2nd
				if (second == 0) {
					status |= (1 << 1);
					output_high(SECOND_LED);
				} else {
					status &= (~(1 << 1));
					output_low(SECOND_LED);
				}
        break;
			default:
				// TODO other cases
			  break;
      }
		} else {
			last_button = 255;
		}
	}
}
