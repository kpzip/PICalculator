//===-- PIC16TargetInfo.cpp - PIC16 Target Implementation -----------------===//
//
//                     The LLVM Compiler Infrastructure
//
// This file is distributed under the University of Illinois Open Source
// License. See LICENSE.TXT for details.
//
//===----------------------------------------------------------------------===//

#include "PIC16.h"
//#include "llvm/Module.h"
#include "llvm/MC/TargetRegistry.h"
#include "llvm/TargetParser/Triple.h"

namespace llvm {
  Target &getThePIC16Target() {
    static Target ThePIC16Target;
    return ThePIC16Target;
  }
  
  Target &getTheCooperTarget() {
    static Target TheCooperTarget;
    return TheCooperTarget;
  }
}

extern "C" LLVM_EXTERNAL_VISIBILITY void LLVMInitializePIC16TargetInfo() { 
  llvm::RegisterTarget<llvm::Triple::pic16> X(llvm::getThePIC16Target(), "pic16",
                                  "PIC16 14-bit [experimental]", "PIC16");

  llvm::RegisterTarget<> Y(llvm::getTheCooperTarget(), "cooper", "PIC16 Cooper [experimental]", "Cooper (who is Cooper????)");
}

extern "C" LLVM_EXTERNAL_VISIBILITY void LLVMInitializePIC16TargetMC()     { /* NOT IMPLEMENTED */ }
extern "C" LLVM_EXTERNAL_VISIBILITY void LLVMInitializePIC16Disassembler() { /* NOT IMPLEMENTED */ }
