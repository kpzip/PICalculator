add_llvm_component_group(PIC16)

set(LLVM_TARGET_DEFINITIONS PIC16.td)

tablegen(LLVM PIC16GenRegisterInfo.h.inc -gen-register-info) # -gen-register-desc-header
#tablegen(LLVM PIC16GenRegisterNames.inc -gen-register-enums)
#tablegen(LLVM PIC16GenRegisterInfo.inc -gen-register-desc)
#tablegen(LLVM PIC16GenInstrNames.inc -gen-instr-enums)
tablegen(LLVM PIC16GenInstrInfo.inc -gen-instr-info) # -gen-instr-desc
tablegen(LLVM PIC16GenAsmWriter.inc -gen-asm-writer)
tablegen(LLVM PIC16GenDAGISel.inc -gen-dag-isel)
tablegen(LLVM PIC16GenCallingConv.inc -gen-callingconv)
tablegen(LLVM PIC16GenSubtargetInfo.inc -gen-subtarget)

add_public_tablegen_target(PIC16CommonTableGen)

add_llvm_target(PIC16CodeGen
  #PIC16DebugInfo.cpp
  PIC16InstrInfo.cpp
  PIC16ISelDAGToDAG.cpp
  PIC16ISelLowering.cpp
  PIC16MemSelOpt.cpp
  PIC16MCAsmInfo.cpp
  PIC16RegisterInfo.cpp
  PIC16Section.cpp
  PIC16Subtarget.cpp
  PIC16TargetMachine.cpp
  PIC16TargetObjectFile.cpp
  PIC16SelectionDAGInfo.cpp
  
  DEPENDS
  intrinsics_gen
  
  LINK_COMPONENTS
  PIC16Info
  AsmPrinter
  CodeGen
  CodeGenTypes
  Core
  MC
  SelectionDAG
  Support
  Target
  
  ADD_TO_COMPONENT
  PIC16
  )

add_subdirectory(TargetInfo)
