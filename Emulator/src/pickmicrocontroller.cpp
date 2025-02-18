/*
 * pickmicrocontroller.cpp
 *
 *  Created on: Feb 17, 2025
 *      Author: kpzip
 */
#include "pickmicrocontroller.h"
#include "./ui_pick_microcontroller.h"

PickMicrocontroller::PickMicrocontroller(QWidget *parent)
	: QDialog(parent)
	, ui(new Ui::Dialog)
{
	ui->setupUi(this);
}

PickMicrocontroller::~PickMicrocontroller() {
	delete ui;
}



