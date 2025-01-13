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

#define DEBUG_TYPE "pic16-subtarget"

#include "PIC16Subtarget.h"

#include "llvm/Target/TargetMachine.h"
#include "PIC16TargetMachine.h"

using namespace llvm;

#define GET_SUBTARGETINFO_ENUM
#define GET_SUBTARGETINFO_TARGET_DESC
#define GET_SUBTARGETINFO_CTOR
#define GET_SUBTARGETINFO_MC_DESC
#include "PIC16GenSubtargetInfo.inc"

PIC16Subtarget::PIC16Subtarget(const Triple &TT, const std::string &CPU,
        const std::string &FS, const PIC16TargetMachine &TM, bool Cooper)
  : PIC16GenSubtargetInfo(TT, CPU, CPU, FS),
    IsCooper(Cooper)
{

  // Parse features string.
  ParseSubtargetFeatures("generic", CPU, FS);
}
