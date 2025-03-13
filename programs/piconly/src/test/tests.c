/*
 * tests.c
 *
 *  Created on: Mar 2, 2025
 *      Author: kpzip
 */
#include <string.h>
#include <stdio.h>

#include "test_defs.h"
#include "../definitions.h"
#include "main.h"

#undef main

void main() {
	// Tests go here

	// Test Expression Parsing
	char *input = "2";
	strcpy(graph_eq_buffer, input);
	graph_eq_write_pointer = strlen(input);
	regenerate_graph_data();
	enable_graph_mode();
	graph_eq_buffer[graph_eq_write_pointer] = '\0';
	uint8_t final_val = graph_data_1[47] >> 2;
	printf("%d\n", final_val);
}




