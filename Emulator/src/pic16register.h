/*
 * pic16register.h
 *
 *  Created on: Jan 23, 2025
 *      Author: max
 */

#ifndef SRC_PIC16REGISTER_H_
#define SRC_PIC16REGISTER_H_

#include <stdint.h>

template <class Machine>
class PIC16Register {
public:
	virtual void write(Machine* machine, uint8_t value) = 0;
	virtual uint8_t read(Machine* machine) = 0;

	virtual ~PIC16Register() {}
};

template <class Machine>
class PIC16ValueRegister : PIC16Register<Machine> {
private:
	uint8_t value;

public:
	virtual void write(Machine* machine, uint8_t value) override { this->value = value; }
	virtual uint8_t read(Machine* machine) override { return this->value; }
};

template <class Machine>
class PIC16TransparentRegister : PIC16Register<Machine> {
private:
	PIC16Register<Machine> *pointee;

public:
	PIC16TransparentRegister(PIC16Register<Machine> *m_pointee) : pointee(m_pointee) {}
	virtual void write(Machine* machine, uint8_t value) override { *pointee = value; }
	virtual uint8_t read(Machine* machine) override { return *pointee; }
};

template <class Machine>
class PIC16ZeroRegister : PIC16Register<Machine> {
public:
	virtual void write(Machine* machine, uint8_t value) override {}
	virtual uint8_t read(Machine* machine) override { return 0; }
};

template <class Machine>
class PIC16PointerRegister : PIC16Register<Machine> {
private:
	PIC16Register<Machine> *FSR;
	PIC16Register<Machine> *STATUS;

public:
	PIC16PointerRegister(PIC16Register<Machine> *m_FSR,
			PIC16Register<Machine> *m_STATUS) : FSR(m_FSR), STATUS(m_STATUS) {}
	virtual void write(Machine *machine, uint8_t value) override;
	virtual uint8_t read(Machine *machine) override;
};

#endif /* SRC_PIC16REGISTER_H_ */
