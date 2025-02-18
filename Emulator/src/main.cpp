#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <stdlib.h>

#include "pic16machine.h"
#include "pic16instructionset.h"
#include "C74.h"
#include "mainwindow.h"

#include <QApplication>
#include <QDebug>

int main(int argc, char *argv[]) {

	qDebug() << "Main";

	QApplication app(argc, argv);

	MainWindow w;
	w.show();

	return app.exec();

//	PIC16C74 *pic = new PIC16C74();
//
//	pic->ioReg()->PORTB = 0xAA;
//	pic->ioReg()->FSR = 0x06;
//	pic->ioReg()->STATUS |= 0b10000000;
//	uint8_t *accessed = pic->getRegFile(0x00);
//	printf("PORTB: %d\n", *accessed);
//	*accessed += 1;
//	printf("PORTB: %d\n", *accessed);

}
