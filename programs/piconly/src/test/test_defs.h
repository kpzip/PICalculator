/*
 * test_defs.h
 *
 *  Created on: Mar 2, 2025
 *      Author: kpzip
 */

#ifndef SRC_TEST_DEFS_H_
#define SRC_TEST_DEFS_H_

#include <stdio.h>
#include "../definitions.h"

#define main pic_main
#define static

void set_tris_a(uint8_t tris);
void set_tris_b(uint8_t tris);
void set_tris_c(uint8_t tris);
void set_tris_d(uint8_t tris);
void set_tris_e(uint8_t tris);

void output_high(uint16_t pin);
void output_low(uint16_t pin);
void output_bit(uint16_t pin, uint8_t value);

void disable_interrupts(uint16_t value);

void delay_ms(uint16_t delay);

void setup_spi(uint32_t options);
void spi_write(uint8_t val);
uint8_t spi_read(uint8_t val);

uint8_t input(uint16_t pin);

#define PIN_A0  40
#define PIN_A1  41
#define PIN_A2  42
#define PIN_A3  43
#define PIN_A4  44
#define PIN_A5  45

#define PIN_B0  48
#define PIN_B1  49
#define PIN_B2  50
#define PIN_B3  51
#define PIN_B4  52
#define PIN_B5  53
#define PIN_B6  54
#define PIN_B7  55

#define PIN_C0  56
#define PIN_C1  57
#define PIN_C2  58
#define PIN_C3  59
#define PIN_C4  60
#define PIN_C5  61
#define PIN_C6  62
#define PIN_C7  63

#define PIN_D0  64
#define PIN_D1  65
#define PIN_D2  66
#define PIN_D3  67
#define PIN_D4  68
#define PIN_D5  69
#define PIN_D6  70
#define PIN_D7  71

#define PIN_E0  72
#define PIN_E1  73
#define PIN_E2  74

#define FALSE 0
#define TRUE 1

#define GLOBAL                    0x0BC0
#define PERIPH                    0x0B40
#define INT_RTCC                  0x000B20
#define INT_RB                    0x00FF0B08
#define INT_EXT_L2H               0x50000B10
#define INT_EXT_H2L               0x60000B10
#define INT_EXT                   0x000B10
#define INT_AD                    0x008C40
#define INT_TBE                   0x008C10
#define INT_RDA                   0x008C20
#define INT_TIMER1                0x008C01
#define INT_TIMER2                0x008C02
#define INT_CCP1                  0x008C04
#define INT_CCP2                  0x008D01
#define INT_SSP                   0x008C08
#define INT_PSP                   0x008C80
#define INT_TIMER0                0x000B20

#define SPI_DISABLED             0x00
#define SPI_MASTER               0x20
#define SPI_SLAVE                0x24
#define SPI_SCK_IDLE_HIGH        0x10
#define SPI_SCK_IDLE_LOW         0x00
#define SPI_CLK_DIV_4            0x00
#define SPI_CLK_DIV_16           0x01
#define SPI_CLK_DIV_64           0x02
#define SPI_CLK_T2               0x03
#define SPI_SS_DISABLED          0x01

#endif /* SRC_TEST_DEFS_H_ */
