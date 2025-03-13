/*
 * main.h
 *
 *  Created on: Mar 2, 2025
 *      Author: kpzip
 */

#ifndef SRC_MAIN_H_
#define SRC_MAIN_H_

#include "../definitions.h"

extern char expr_buffer[EXPR_BUF_LEN];
extern uint8_t expr_write_pointer;

extern char graph_eq_buffer[EXPR_BUF_LEN];
extern uint8_t graph_eq_write_pointer;

extern uint8_t graph_data_1[48];
extern uint8_t graph_data_2[48];

void pic_main();
void simplify_expr();
void regenerate_graph_data();
void enable_graph_mode();



#endif /* SRC_MAIN_H_ */
