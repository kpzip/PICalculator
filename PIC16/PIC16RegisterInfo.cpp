//===- PIC16RegisterInfo.cpp - PIC16 Register Information -----------------===//
//
//                     The LLVM Compiler Infrastructure
//
// This file is distributed under the University of Illinois Open Source 
// License. See LICENSE.TXT for details.
//
//===----------------------------------------------------------------------===//
//
// This file contains the PIC16 implementation of the TargetRegisterInfo class.
//
//===----------------------------------------------------------------------===//

#define DEBUG_TYPE "pic16-reg-info"

#include "PIC16.h"
#include "PIC16RegisterInfo.h"
#include "PIC16Subtarget.h"
#include "llvm/ADT/BitVector.h"
#include "llvm/Support/ErrorHandling.h"
#include "llvm/CodeGen/MachineFunction.h"

#define GET_REGINFO_TARGET_DESC
#define GET_REGINFO_MC_DESC
#include "PIC16GenRegisterInfo.h.inc"

#define GET_INSTRINFO_ENUM
#include "PIC16GenInstrInfo.inc"

using namespace llvm;

PIC16RegisterInfo::PIC16RegisterInfo(const TargetInstrInfo &tii,
                                     const PIC16Subtarget &st)
  : PIC16GenRegisterInfo(PIC16::ADJCALLSTACKDOWN, PIC16::ADJCALLSTACKUP),
    TII(tii),
    ST(st) {}

/// PIC16 Callee Saved Registers
const MCPhysReg* PIC16RegisterInfo::getCalleeSavedRegs(const MachineFunction *MF) const {
  static const MCPhysReg CSR_SaveList[] = { 0 }; // HACK
  return CSR_SaveList;
}

BitVector PIC16RegisterInfo::getReservedRegs(const MachineFunction &MF) const {
  BitVector Reserved(getNumRegs());
  return Reserved;
}

bool PIC16FrameLowering::hasFP(const MachineFunction &MF) const {
  return false;
}

bool PIC16RegisterInfo::
eliminateFrameIndex(MachineBasicBlock::iterator II, int SPAdj, unsigned balls,
                    RegScavenger *RS) const
{ return false; /* NOT YET IMPLEMENTED */ }

void PIC16FrameLowering::emitPrologue(MachineFunction &MF, MachineBasicBlock &MBB) const
{    /* NOT YET IMPLEMENTED */  }

void PIC16FrameLowering::emitEpilogue(MachineFunction &MF, MachineBasicBlock &MBB) const
{    /* NOT YET IMPLEMENTED */  }

//int PIC16RegisterInfo::
//getDwarfRegNum(unsigned RegNum, bool isEH) const {
//  llvm_unreachable("Not keeping track of debug information yet!!");
//  return -1;
//}

Register PIC16RegisterInfo::getFrameRegister(const MachineFunction &MF) const {
  llvm_unreachable("PIC16 Does not have any frame register");
  return 0;
}

//unsigned PIC16RegisterInfo::getRARegister() const {
//  llvm_unreachable("PIC16 Does not have any return address register");
//  return 0;
//}

// This function eliminates ADJCALLSTACKDOWN,
// ADJCALLSTACKUP pseudo instructions
MachineBasicBlock::iterator PIC16FrameLowering::
eliminateCallFramePseudoInstr(MachineFunction &MF, MachineBasicBlock &MBB,
                              MachineBasicBlock::iterator I) const {
  // Simply discard ADJCALLSTACKDOWN,
  // ADJCALLSTACKUP instructions.
  MBB.erase(I);
  return 0;
}

PIC16FrameLowering::PIC16FrameLowering()
    : TargetFrameLowering(TargetFrameLowering::StackGrowsUp, Align(8), 0) {}
