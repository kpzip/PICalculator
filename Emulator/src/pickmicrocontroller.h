/*
 * pickmicrocontroller.h
 *
 *  Created on: Feb 17, 2025
 *      Author: kpzip
 */

#ifndef SRC_PICKMICROCONTROLLER_H_
#define SRC_PICKMICROCONTROLLER_H_


#include <QDialog>

QT_BEGIN_NAMESPACE
namespace Ui {
	class Dialog;
}
QT_END_NAMESPACE

class PickMicrocontroller : public QDialog {
    Q_OBJECT
public:
	PickMicrocontroller(QWidget *parent = nullptr);
	~PickMicrocontroller();

private:
    Ui::Dialog *ui;
};


#endif /* SRC_PICKMICROCONTROLLER_H_ */
