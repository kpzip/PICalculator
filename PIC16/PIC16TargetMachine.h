//===-- PIC16TargetMachine.h - Define TargetMachine for PIC16 ---*- C++ -*-===//
//
//                     The LLVM Compiler Infrastructure
//
// This file is distributed under the University of Illinois Open Source
// License. See LICENSE.TXT for details.
//
//===----------------------------------------------------------------------===//
//
// This file declares the PIC16 specific subclass of TargetMachine.
//
//===----------------------------------------------------------------------===//


#ifndef PIC16_TARGETMACHINE_H
#define PIC16_TARGETMACHINE_H

#include "PIC16InstrInfo.h"
#include "PIC16ISelLowering.h"
#include "PIC16SelectionDAGInfo.h"
#include "PIC16RegisterInfo.h"
#include "PIC16Subtarget.h"
#include "TargetInfo/PIC16TargetInfo.h"
#include "llvm/IR/DataLayout.h"
#include "llvm/CodeGen/MachineFrameInfo.h"
#include "llvm/Target/TargetMachine.h"
#include "llvm/CodeGen/TargetPassConfig.h"

namespace llvm {

/// PIC16TargetMachine
///
class PIC16TargetMachine : public LLVMTargetMachine {
  PIC16Subtarget        Subtarget;
  const char           *DataLayout;       // Calculates type size & alignment
  PIC16InstrInfo        InstrInfo;
  PIC16TargetLowering   TLInfo;
  PIC16SelectionDAGInfo TSInfo;

  // PIC16 does not have any call stack frame, therefore not having 
  // any PIC16 specific FrameInfo class.
  PIC16FrameLowering       FrameInfo;

public:
  PIC16TargetMachine(const Target &T, const Triple &TT,
			StringRef CPU, StringRef FS,
			const TargetOptions &Options,
			std::optional<Reloc::Model> RM,
			std::optional<CodeModel::Model> CM,
			CodeGenOptLevel OL, bool JIT);

  virtual const PIC16FrameLowering  *getFrameInfo() const { return &FrameInfo; }
  virtual const PIC16InstrInfo      *getInstrInfo() const  { return &InstrInfo; }
  virtual const char                *getTargetData()       { return DataLayout; }
  virtual const PIC16Subtarget      *getSubtargetImpl(const Function &F) const { return &Subtarget; }
  const PIC16Subtarget      *getSubtargetImpl() const { return &Subtarget; } // HACK!!!!!
  const PIC16Subtarget      *getSubtarget() const { return &Subtarget; } // HACK!!!!!

  virtual const PIC16RegisterInfo *getRegisterInfo() const { 
    return &(InstrInfo.getRegisterInfo()); 
  }

  virtual const PIC16TargetLowering *getTargetLowering() const { 
    return &TLInfo;
  }

  virtual const PIC16SelectionDAGInfo* getSelectionDAGInfo() const {
    return &TSInfo;
  }

  TargetPassConfig *createPassConfig(PassManagerBase &PM) override;

//  virtual bool addInstSelector(PassManagerBase &PM,
//                               CodeGenOptLevel OptLevel);
//  virtual bool addPreEmitPass(PassManagerBase &PM, CodeGenOptLevel OptLevel);
}; // PIC16TargetMachine.

class PIC16PassConfig : public TargetPassConfig {
public:
  PIC16PassConfig(PIC16TargetMachine &TM, PassManagerBase &PM)
      : TargetPassConfig(TM, PM) {}

  PIC16TargetMachine &getPIC16TargetMachine() const {
    return getTM<PIC16TargetMachine>();
  }

  virtual void addIRPasses() {}
  virtual bool addInstSelector() override;
  virtual void addPreSched2() {}
  virtual void addPreEmitPass() override;
};

} // end namespace llvm

#endif
