//===- PIC16Subtarget.cpp - PIC16 Subtarget Information -------------------===//
//
//                     The LLVM Compiler Infrastructure
//
// This file is distributed under the University of Illinois Open Source 
// License. See LICENSE.TXT for details.
//
//===----------------------------------------------------------------------===//
//
// This file implements the PIC16 specific subclass of TargetSubtarget.
//
//===----------------------------------------------------------------------===//

#include "PIC16Subtarget.h"

#include "llvm/Target/TargetMachine.h"

using namespace llvm;

PIC16Subtarget::PIC16Subtarget(const StringRef &CPU, const StringRef &TuneCPU,
                               const StringRef &FS, const TargetMachine &TM, bool Cooper)
  : PIC16GenSubtargetInfo(TM.getTargetTriple(), CPU, TuneCPU, FS),
    IsCooper(Cooper)
{

  // Parse features string.
  ParseSubtargetFeatures("generic", TuneCPU, FS);
}
