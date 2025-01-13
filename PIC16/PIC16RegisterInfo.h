//===- PIC16RegisterInfo.h - PIC16 Register Information Impl ----*- C++ -*-===//
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

#ifndef PIC16REGISTERINFO_H
#define PIC16REGISTERINFO_H

#include "llvm/CodeGen/TargetRegisterInfo.h"
#include "llvm/CodeGen/TargetFrameLowering.h"

#define GET_REGINFO_HEADER
#define GET_REGINFO_ENUM
#include "PIC16GenRegisterInfo.h.inc"

namespace llvm {

// Forward Declarations.
  class PIC16Subtarget;
  class TargetInstrInfo;

class PIC16RegisterInfo : public PIC16GenRegisterInfo {
private:
	const TargetInstrInfo &TII;
	const PIC16Subtarget &ST;
  
public:
    PIC16RegisterInfo(const TargetInstrInfo &tii, 
                      const PIC16Subtarget &st);


  //------------------------------------------------------
  // Pure virtual functions from TargetRegisterInfo
  //------------------------------------------------------

  // PIC16 callee saved registers
  const MCPhysReg *getCalleeSavedRegs(const MachineFunction *MF = 0) const override;

  BitVector getReservedRegs(const MachineFunction &MF) const override;
  //virtual bool hasFP(const MachineFunction &MF) const;

  bool eliminateFrameIndex(MachineBasicBlock::iterator MI,
                               int SPAdj, unsigned balls, RegScavenger *RS = nullptr) const override;
                                   
  // void eliminateCallFramePseudoInstr(MachineFunction &MF,
  //                                    MachineBasicBlock &MBB,
  //                                    MachineBasicBlock::iterator I) const;

  //virtual void emitPrologue(MachineFunction &MF) const;
  //virtual void emitEpilogue(MachineFunction &MF, MachineBasicBlock &MBB) const;
//  int getDwarfRegNum(unsigned RegNum, bool isEH) const override;
  Register getFrameRegister(const MachineFunction &MF) const override;
//  unsigned getRARegister() const override;
};

/// Utilities for creating function call frames.
class PIC16FrameLowering : public TargetFrameLowering {
public:
  explicit PIC16FrameLowering();

public:
  void emitPrologue(MachineFunction &MF, MachineBasicBlock &MBB) const override;
  void emitEpilogue(MachineFunction &MF, MachineBasicBlock &MBB) const override;
  bool hasFP(const MachineFunction &MF) const override;
  // bool spillCalleeSavedRegisters(MachineBasicBlock &MBB,
  //                                MachineBasicBlock::iterator MI,
  //                                ArrayRef<CalleeSavedInfo> CSI,
  //                                const TargetRegisterInfo *TRI) const override;
  // bool
  // restoreCalleeSavedRegisters(MachineBasicBlock &MBB,
  //                             MachineBasicBlock::iterator MI,
  //                             MutableArrayRef<CalleeSavedInfo> CSI,
  //                             const TargetRegisterInfo *TRI) const override;
  // bool hasReservedCallFrame(const MachineFunction &MF) const override;
  // bool canSimplifyCallFramePseudos(const MachineFunction &MF) const override;
  // void determineCalleeSaves(MachineFunction &MF, BitVector &SavedRegs,
  //                           RegScavenger *RS = nullptr) const override;
  MachineBasicBlock::iterator
  eliminateCallFramePseudoInstr(MachineFunction &MF, MachineBasicBlock &MBB,
                                MachineBasicBlock::iterator MI) const override;
};

} // end namespace llvm

#endif
