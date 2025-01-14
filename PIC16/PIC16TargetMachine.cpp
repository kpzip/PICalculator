//===-- PIC16TargetMachine.cpp - Define TargetMachine for PIC16 -----------===//
//
//                     The LLVM Compiler Infrastructure
//
// This file is distributed under the University of Illinois Open Source 
// License. See LICENSE.TXT for details.
//
//===----------------------------------------------------------------------===//
//
// Top-level implementation for the PIC16 target.
//
//===----------------------------------------------------------------------===//

#include "PIC16.h"
#include "PIC16MCAsmInfo.h"
#include "PIC16TargetMachine.h"
#include "llvm/IR/PassManager.h"
#include "llvm/CodeGen/Passes.h"
#include "llvm/MC/TargetRegistry.h"

using namespace llvm;

static const char *PIC16DataLayout =
	"e-p:16:8:8-i8:8:8-i16:8:8-i32:8:8-n8";

extern "C" void LLVMInitializePIC16Target() {
  // Register the target. Curretnly the codegen works for
  // enhanced pic16 mid-range.
  RegisterTargetMachine<PIC16TargetMachine> X(getThePIC16Target());
  //RegisterAsmInfo<PIC16MCAsmInfo> A(getThePIC16Target());
}

static Reloc::Model getEffectiveRelocModel(std::optional<Reloc::Model> RM) {
  return RM.value_or(Reloc::Static);
}

// PIC16TargetMachine - Enhanced PIC16 mid-range Machine. May also represent
// a Traditional Machine if 'Trad' is true.
PIC16TargetMachine::PIC16TargetMachine(const Target &T, const Triple &TT,
										StringRef CPU, StringRef FS,
										const TargetOptions &Options,
										std::optional<Reloc::Model> RM,
										std::optional<CodeModel::Model> CM,
										CodeGenOptLevel OL, bool JIT)
: LLVMTargetMachine(T, StringRef(PIC16DataLayout), TT, CPU, FS, Options,
		getEffectiveRelocModel(RM),
        getEffectiveCodeModel(CM, CodeModel::Small), OL),
  Subtarget(TT, CPU.str(), FS.str(), *this, false),
  DataLayout(PIC16DataLayout),
  InstrInfo(*this), TLInfo(*this), TSInfo(*this),
  //FrameInfo(TargetFrameInfo::StackGrowsUp, 8, 0) { } //HACK!!!
  FrameInfo() {

  // Maybe do something like this->TLOF = std::make_unique<AVRTargetObjectFile>();
  // initAsmInfo();
}


bool PIC16PassConfig::addInstSelector() {
  // Install an instruction selector.
  addPass(createPIC16ISelDag(getPIC16TargetMachine(), getOptLevel()));
  return false;
}

void PIC16PassConfig::addPreEmitPass() {
  addPass(createPIC16MemSelOptimizerPass());
}

TargetPassConfig *PIC16TargetMachine::createPassConfig(PassManagerBase &PM) {
  return new PIC16PassConfig(*this, PM);
}
