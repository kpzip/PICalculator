#include <16C74.h>

#use delay(clock=20M, RESTART_WDT)

#define DSS PIN_A4

void write_spi(unsigned char data, unsigned char DI) {

	output_high(DSS);
	spi_write(0xF8 | (DI << 1));
	spi_write(data & 0xF0);
	spi_write((data << 4) & 0xF0);
	output_low(DSS);

	delay_ms(50);
}

void main() {
	set_tris_a(0x00); // Set all of PORT A to output
	set_tris_d(0x00);
	set_tris_e(0x00);

	output_low(DSS);

	setup_spi(SPI_MASTER | SPI_SCK_IDLE_HIGH);

	while (TRUE) {

		write_spi(0x30, 0); // Function set: 8 bit interface, basic instruction set
		write_spi(0x08, 0); // Display status: Everything off
		write_spi(0x10, 0); // Cursor: Move left (?)
		write_spi(0x0F, 0); // Display status: Display, cursor, and blink on
		write_spi(0x01, 0); // Clear
		write_spi(0x06, 0); // Make cursor move right
		write_spi(0x80, 0); // Home the cursor (unneccesary)

		for (unsigned char i = 0; i <= 254; i++) {
			write_spi(i, 1);
		}
	}
}
