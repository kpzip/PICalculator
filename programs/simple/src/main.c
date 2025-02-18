#include <stdint.h>

// Bit Vector: each 6 bits represents the y position for each of the 128 x positions.
static uint8_t graphdata[96] = { 0 };

static uint8_t scan_counter = 0;

static uint8_t last_button = 0;

int main() {
	// Main calculator loop
	while (1) {
		// Check for input: scan across different button ranks and determine the button pressed if any
		for (scan_counter = 0; scan_counter < 8; scan_counter++) {
			// TODO scanning and reading
			last_button = 0;
		}
		if (last_button != 0) {

		}
		graphdata = 0;
	}
}
