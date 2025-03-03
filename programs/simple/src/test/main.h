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

void pic_main();
void simplify_expr();



#endif /* SRC_MAIN_H_ */
