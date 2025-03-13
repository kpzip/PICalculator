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
	char *input = "10";
	strcpy(graph_eq_buffer, input);
	graph_eq_write_pointer = strlen(input);
	regenerate_graph_data();
	graph_eq_buffer[graph_eq_write_pointer] = '\0';
	printf("%s\n", graph_eq_buffer);
}




