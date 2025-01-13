//===-- PIC16TargetInfo.h - PIC16 Target Implementation ---------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_PIC16_TARGET_INFO_H
#define LLVM_PIC16_TARGET_INFO_H

namespace llvm {
class Target;

Target &getThePIC16Target();
Target &getTheCooperTarget();
} // namespace llvm

#endif // LLVM_PIC16_TARGET_INFO_H
