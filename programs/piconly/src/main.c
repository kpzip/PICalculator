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

// Comment out to disable Joao mode
#define JOAO_MODE

// Calculator global state
// Bit Vector: each 6 bits represents the y position for each of the 128 x positions.
static uint8_t graph_data_1[48] = { 0 };
static uint8_t graph_data_2[48] = { 0 };

static uint8_t row_counter = 0;
static uint8_t column_counter = 0;
static uint8_t last_button = 255;

// Bit 0: Radians = 0, Degrees = 1; Bit 1: Second mode, 0 = off, Bit2: Alpha mode, 0 = off
static uint8_t status = 0;

// 0: Normal 1: Graph Eq Entry 2: Graph View 3: Joao Viewer
static uint8_t mode = 0;

static char expr_buffer[EXPR_BUF_LEN] = { 0 };
static char graph_eq_buffer[EXPR_BUF_LEN] = { 0 };
static uint8_t expr_write_pointer = 0;
static uint8_t graph_eq_write_pointer = 0;

typedef union {
	uint32_t val;
	uint8_t bytes[4];
} packer;

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

	delay_ms(1);
}

void display_data(uint8_t data) {
	output_low(DSS);
	spi_write(data & 0xF0);
	spi_write((data << 4) & 0xF0);
	output_high(DSS);
}

uint16_t pow(uint16_t base, uint16_t exponent) {
	uint16_t ret = 1;
	for (uint8_t i = 0; i < exponent; i++) {
		ret *= base;
	}
	return ret;
}

uint16_t char_to_num(char c) {
	return c - 48;
}

void write_num(uint16_t num) {

    do {
    	expr_buffer[expr_write_pointer++] = (num % 10) + '0';
        num /= 10;
    } while (num > 0);

    uint8_t start = 0;
    uint8_t end = expr_write_pointer - 1;
    while (start < end) {
        char temp = expr_buffer[start];
        expr_buffer[start] = expr_buffer[end];
        expr_buffer[end] = temp;
        start++;
        end--;
    }
}

void enable_normal_mode() {
	// Set mode and clear
	display_command(0x30, 0);
	display_command(0x0E, 0);
	display_command(0x01, 0);

	delay_ms(50);

	// Write the current normal mode expression
	for (uint8_t i = 0; i < expr_write_pointer; i++) {
		display_command(expr_buffer[i], 1);
	}
}

void enable_graph_eq_mode() {
	// Set mode and clear
	display_command(0x30, 0);
	display_command(0x01, 0);

	// Write the function
	display_command('Y', 1);
	display_command('=', 1);
	for (uint8_t i = 0; i < graph_eq_write_pointer; i++) {
		display_command(graph_eq_buffer[i], 1);
	}
}

#separate
void enable_graph_mode() {
	// Set Mode
	
	//display_command(0x01, 0);
	//display_command(0x30, 0);
	display_command(0x0C, 0);
	display_command(0x34, 0);
	display_command(0x36, 0);

	for (uint8_t horizontal_addr = 0; horizontal_addr < 8; horizontal_addr++) {

		// Memory-perf tradeoff. can change this based on which is more important later.
		// Stores the vertical positions of the dots
		uint8_t values[16];
		packer p = {0};

		for (uint8_t group = 0; group < 4; group++) {
			uint8_t index = 3 * (group + 4 * horizontal_addr);
			if (index < 48) {
				p.bytes[0] = graph_data_1[index + 0];
				p.bytes[1] = graph_data_1[index + 1];
				p.bytes[2] = graph_data_1[index + 2];
			} else {
				index -= 48;
				p.bytes[0] = graph_data_2[index + 0];
				p.bytes[1] = graph_data_2[index + 1];
				p.bytes[2] = graph_data_2[index + 2];
			}
			values[group * 4] = p.val & 0b00111111;
			values[group * 4 + 1] = (p.val >> 6) & 0b00111111;
			values[group * 4 + 2] = (p.val >> 12) & 0b00111111;
			values[group * 4 + 3] = (p.val >> 18) & 0b00111111;
		}

		for (uint8_t vertical_addr = 0; vertical_addr < 64; vertical_addr++) {
			uint8_t lsb = 0;
			uint8_t msb = 0;
			uint8_t bit_idx;
			for (bit_idx = 0; bit_idx < 8; bit_idx++) {
				if (values[bit_idx] == vertical_addr) {
					msb |= (0x80 >> bit_idx);
				}
			}
			for (bit_idx = 0; bit_idx < 8; bit_idx++) {
				if (values[bit_idx + 8] == vertical_addr) {
					lsb |= (0x80 >> bit_idx);
				}
			}

			//printf("Vertical Address: %d\n", vertical_addr);
			//printf("Horizontal Address: %d\n", horizontal_addr);

			//printf("LSB: %d\n", lsb);
			//printf("MSB: %d\n", msb);
			
			// X axis
			if (vertical_addr == 32) {
				msb = 0xFF;
				lsb = 0xFF;
			}

			// Set Address
			display_command(0x80 + (vertical_addr >= 32 ? vertical_addr - 32 : vertical_addr), 0);
			display_command(0x80 + (vertical_addr >= 32 ? horizontal_addr + 8 : horizontal_addr), 0);
			
			// Send Data
			display_command(msb, 1);
			display_command(lsb, 1);
		}
	}
}

#ifdef JOAO_MODE
void enable_joao_viewer() {
	
}
#endif

uint16_t parse_from_buffer(char *buf, uint8_t begin, uint8_t end) {
	uint8_t digit_val;
	uint8_t i;	
	uint16_t ret = 0;
	
	for (i = begin; i < end; i++) {
		digit_val = char_to_num(buf[i]);
		if (digit_val == 0xFF) {
			return 0xFFFF;
		}
		ret += pow(10, end - (i + 1)) * digit_val;
	}
	return ret;
}

#separate
void regenerate_graph_data() {
	// form: y=mx+b
	uint8_t m = 0;
	uint8_t b = 0;
	uint8_t i;

	// Can be unionized if need be
	uint8_t pos;
	uint8_t digit_val;
	for (pos = 0; graph_eq_buffer[pos] != 'X'; pos++) {
		if (pos >= graph_eq_write_pointer - 1) {
			goto nox;
		}
	}
	
	m = parse_from_buffer(graph_eq_buffer, 0, pos);
	if (m == 0xFF) {
		goto err;
	}
	pos++;
	
	if (pos < graph_eq_write_pointer - 1 && graph_eq_buffer[pos] == '+') {
		pos++;
		b = parse_from_buffer(graph_eq_buffer, pos, graph_eq_write_pointer);
		if (b == 0xFF) {
			goto err;
		}
	}
	draw:
	;

	uint8_t val;
	uint8_t index;
	packer word;

	word.val = 0;

	// X position loop
	// slow. optimize by calculating one value and going from there
	for (i = 0; i < 128; i++) {
		val = 63 - (((char)i - 63) * m + b + 31);
		if (val > 63) {
			val = 32;
		}
		word.val |= ((uint32_t)val << ((i % 4) * 6));
		// Copy first 3 bytes of word into graphdata
		if (i % 4 == 3) {
			index = (i/4) * 3;
			if (index >= 48) {
				index -= 48;
				graph_data_2[index] = word.bytes[0];
				graph_data_2[index + 1] = word.bytes[1];
				graph_data_2[index + 2] = word.bytes[2];
			} else {
				graph_data_1[index] = word.bytes[0];
				graph_data_1[index + 1] = word.bytes[1];
				graph_data_1[index + 2] = word.bytes[2];
			}
			word.val = 0;
		}
	}
	
	return;
	nox:
	b = parse_from_buffer(graph_eq_buffer, 0, graph_eq_write_pointer);
	if (b == 0xFF) {
		goto err;
	}
	goto draw;
	err:
	graph_eq_buffer[0] = 'e';
	graph_eq_buffer[1] = 'r';
	graph_eq_buffer[2] = 'r';
	graph_eq_write_pointer = 3;
	return;
}

#separate
void simplify_expr() {
	uint8_t i;
	uint8_t first_operator = 0xFF;
	uint8_t second_operator = 0xFF;
	for (i = 0; i < expr_write_pointer; i++) {
		switch(expr_buffer[i]) {
		case '+':
		case '-':
		case '*':
		case '/':
		case '^':
			if (first_operator == 0xFF) {
				first_operator = i;
			} else {
				second_operator = i;
			}
			break;
		default:
			break;
		}
	}

	// No need to simplify if there are no operators!
	// The only way they can be equal is if they are both 0xFF
	if (first_operator == second_operator) {
		return;
	}

	// Maybe make these larger?
	uint16_t first_number = 0;
	uint16_t second_number = 0;
	uint16_t third_number = 0;

	uint8_t digit_val;
	
	first_number = parse_from_buffer(expr_buffer, 0, first_operator);
	if (first_number == 0xFFFF) goto err2;
	if (second_operator != 0xFF) {
		second_number = parse_from_buffer(expr_buffer, first_operator + 1, second_operator);
		if (second_number == 0xFFFF) goto err2;
		third_number = parse_from_buffer(expr_buffer, second_operator + 1, expr_write_pointer);
		if (third_number == 0xFFFF) goto err2;
	}
	else {
		second_number = parse_from_buffer(expr_buffer, first_operator + 1, expr_write_pointer);
		if (second_number == 0xFFFF) goto err2;
	}

	first_operator = expr_buffer[first_operator];
	if (second_operator != 0xFF) second_operator = expr_buffer[second_operator];
	uint16_t result;
	expr_write_pointer = 0;
	if (first_operator == '*') {
		// First operator precedence
		result = first_number * second_number;
		if (second_operator != 0xFF) switch (second_operator) {
		case '+':
			result += third_number;
			break;
		case '-':
			result -= third_number;
			break;
		case '*':
			result *= third_number;
			break;
		case '/':
			//if (third_number == 0) {
			//	goto err2;
			//}
			result /= third_number;
			break;
		case '^':
			result = pow(result, third_number);
			break;
		}

	} else if (first_operator == '/') {
		// First operator precedence
		//if (second_number == 0) {
		//	goto err2;
		//}
		result = first_number / second_number;
		if (second_operator != 0xFF) switch (second_operator) {
		case '+':
			result += third_number;
			break;
		case '-':
			result -= third_number;
			break;
		case '*':
			result *= third_number;
			break;
		case '/':
			//if (third_number == 0) {
			//	goto err2;
			//}
			result /= third_number;
			break;
		case '^':
			result = pow(result, third_number);
			break;
		}
	} else if (first_operator == '^') {
		result = pow(first_number, second_number);
		if (second_operator != 0xFF) switch (second_operator) {
		case '+':
			result += third_number;
			break;
		case '-':
			result -= third_number;
			break;
		case '*':
			result *= third_number;
			break;
		case '/':
			//if (third_number == 0) {
			//	goto err2;
			//}
			result /= third_number;
			break;
		case '^':
			result = pow(result, third_number);
			break;
		}
	} else if (second_operator != 0xFF) {
		// Second operator precedence
		switch (second_operator) {
		case '+':
			result = second_number + third_number;
			break;
		case '-':
			result = second_number - third_number;
			break;
		case '*':
			result = second_number * third_number;
			break;
		case '/':
			//if (third_number == 0) {
			//	goto err2;
			//}
			result = second_number / third_number;
			break;
		case '^':
			result = pow(first_number, second_number);
		}
		switch (first_operator) {
		case '+':
			result += first_number;
			break;
		case '-':
			result = first_number - result;
			break;
		}
	} else {
		switch (first_operator) {
		case '+':
			result = first_number + second_number;
			break;
		case '-':
			result = first_number - second_number;
			break;
		}
	}
	write_num(result);
	return;
err2:
	expr_buffer[0] = 'e';
	expr_buffer[1] = 'r';
	expr_buffer[2] = 'r';
	expr_write_pointer = 3;
	return;

}

void write_character(char c) {
	if (mode == 0) {
		expr_buffer[expr_write_pointer++] = c;
		if (expr_write_pointer != 0) {
			display_command(expr_buffer[expr_write_pointer - 1], 1);
		}

		if (expr_write_pointer >= EXPR_BUF_LEN) {
			//display_command(0x01, 0);
			expr_write_pointer = 0;
		}
	} else if (mode == 1) {
		graph_eq_buffer[graph_eq_write_pointer++] = c;
		if (graph_eq_write_pointer != 0) {
			display_command(graph_eq_buffer[graph_eq_write_pointer - 1], 1);
		}

		if (graph_eq_write_pointer >= EXPR_BUF_LEN) {
			//display_command(0x01, 0);
			graph_eq_write_pointer = 0;
		}
	}
}

#separate
void splash_text() {
	display_command('P', 1);
	display_command('I', 1);
	display_command('C', 1);
	display_command('a', 1);
	display_command('l', 1);
	display_command('c', 1);
	display_command('u', 1);
	display_command('l', 1);
	display_command('a', 1);
	display_command('t', 1);
	display_command('o', 1);
	display_command('r', 1);
	display_command(' ', 1);
	display_command(' ', 1);
	display_command(' ', 1);
	display_command(' ', 1);

	display_command('C', 1);
	display_command('o', 1);
	display_command('p', 1);
	display_command('y', 1);
	display_command('r', 1);
	display_command('i', 1);
	display_command('g', 1);
	display_command('h', 1);
	display_command('t', 1);
	display_command(' ', 1);
	display_command('2', 1);
	display_command('0', 1);
	display_command('2', 1);
	display_command('5', 1);
	display_command(' ', 1);
	display_command(' ', 1);

	display_command('v', 1);
	display_command('1', 1);
	display_command('.', 1);
	display_command('1', 1);
	display_command(' ', 1);
	display_command(' ', 1);
	display_command(' ', 1);
	display_command(' ', 1);
	display_command(' ', 1);
	display_command(' ', 1);
	display_command(' ', 1);
	display_command(' ', 1);
	display_command(' ', 1);
	display_command(' ', 1);
	display_command(' ', 1);
	display_command(' ', 1);
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

	// Init the SPI bus
	setup_spi(SPI_MASTER | SPI_SCK_IDLE_HIGH | SPI_CLK_DIV_64);

	// Initialize display
	delay_ms(1);
	display_command(0x30, 0); // Function set: 8 bit interface, basic instruction set
	// display_command(0x08, 0); // Display status: Everything off
	// display_command(0x10, 0); // Cursor: Move left (?)
	display_command(0x0E, 0); // Display status: Display, cursor, and blink on
	display_command(0x01, 0); // Clear
	display_command(0x06, 0); // Make cursor move right
	// display_command(0x80, 0); // Home the cursor (unneccesary)

	// Powerup sequence
	output_high(SECOND_LED);
	output_high(ALPHA_LED);
	output_high(RADIANS_LED);
	output_high(DEGREES_LED);

	// Write PICalculator text
	splash_text();

#ifdef JOAO_MODE
	display_command('J', 1);
	display_command('o', 1);
	display_command('a', 1);
	display_command('o', 1);
	display_command(' ', 1);
	display_command('E', 1);
	display_command('d', 1);
	display_command('i', 1);
	display_command('t', 1);
	display_command('i', 1);
	display_command('o', 1);
	display_command('n', 1);
	display_command(' ', 1);
	display_command(' ', 1);
	display_command(' ', 1);
	display_command(' ', 1);
#endif

	delay_ms(2000);

	output_low(SECOND_LED);
	output_low(ALPHA_LED);
	output_low(RADIANS_LED);
	output_low(DEGREES_LED);
	display_command(0x01, 0);

	// Sync LED
	output_high(RADIANS_LED);

	// Main calculator loop
	while (TRUE) {
		// Check for input: scan across different button ranks and determine the button pressed if any
		uint8_t any_button_pressed = 0;
		for (row_counter = 0; row_counter < 7; row_counter++) {
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
			uint8_t second = (status >> 1) & 1;
			uint8_t alpha = (status >> 2) & 1;
			// Cases are responsible for updating the display
			switch(last_button) {
			// Reset
			case 0:
				expr_write_pointer = 0;
				display_command(0x01, 0); // Clear display
				break;
			// Gamma
			case 31:
				output_low(COSS);
				spi_write(status);
				delay_ms(10);
				status = spi_read(0x00);
				second = (status >> 1) & 1;
				alpha = (status >> 2) & 1;
				uint8_t degrad = status & 1;
				output_bit(SECOND_LED, second);
				output_bit(ALPHA_LED, alpha);
				output_bit(RADIANS_LED, degrad ^ 1);
				output_bit(DEGREES_LED, degrad);
				output_high(COSS);
				break;
			////////////
			// Numpad //
			////////////
			case 3:
				// Numpad 0
				write_character('0');
				break;
			case 8:
				// Numpad 7
				write_character('7');
				break;
			case 9:
				// Numpad 8
				write_character('8');
				break;
			case 10:
				// Numpad 9
				write_character('9');
				break;
			case 14:
				// Numpad 4
				write_character('4');
				break;
			case 15:
				// Numpad 5
				write_character('5');
				break;
			case 16:
				// Numpad 6
				write_character('6');
				break;
			case 20:
				// Numpad 1
				write_character('1');
				break;
			case 21:
				// Numpad 2
				write_character('2');
				break;
			case 22:
				// Numpad 3
				write_character('3');
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
			/////////////////
			// +, -, *, /  //
			/////////////////
			case 29:
				// Add sign
				write_character('+');
				break;
			case 23:
				// Subtract sign
				write_character('-');
				break;
			case 17:
				// Multiplication sign
				write_character('*');
				break;
			case 11:
				// Divide sign
				write_character('/');
				break;
			/////////////////
			//  variables  //
			/////////////////
			case 7:
				write_character('X');
				break;
			/// Other stuff
			case 36:
				// Graph mode
				switch(mode) {
				case 0:
					// Normal Mode
					// Set to Graph eq mode
					enable_graph_eq_mode();
					mode = 1;
					break;
				case 1:
					// Graph Equation Entry
					// Graph the equation and go into the graph view
					regenerate_graph_data();
					enable_graph_mode();
					mode = 2;
					break;
				case 2:
					// Graph View
					// Set back to normal mode
					enable_normal_mode();
					mode = 0;
					break;
#ifdef JOAO_MODE
				case 3:
					// Joao Mode Behavior
					break;
#endif
				default:
					// Shouldnt end up here
					break;
				}

				// TODO
				break;
			case 5:
				// Eval / equals
				if (mode == 0) {
					simplify_expr();
					enable_normal_mode();
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
