#include "mainwindow.h"
#include "./ui_mainwindow.h"
#include "pickmicrocontroller.h"
#include <stdio.h>

MainWindow::MainWindow(QWidget *parent)
    : QMainWindow(parent)
    , ui(new Ui::MainWindow)
{
    ui->setupUi(this);
}

MainWindow::~MainWindow() {
    delete ui;
}

void MainWindow::on_btnSelectUController_clicked() {
	printf("Clicked!\n");
	PickMicrocontroller p(this);
	p.setModal(true);
	p.show();
}


