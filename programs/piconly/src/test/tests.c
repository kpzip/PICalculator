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
	char *input = "10*2*3";
	strcpy(expr_buffer, input);
	expr_write_pointer = strlen(input);
	simplify_expr();
	expr_buffer[expr_write_pointer] = '\0';
	printf("%s\n", expr_buffer);
}




