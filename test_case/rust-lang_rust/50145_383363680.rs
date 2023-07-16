
include/llvm/CodeGen/RuntimeLibcalls.def
118:HANDLE_LIBCALL(POWI_F32, "__powisf2")

lib/CodeGen/SelectionDAG/LegalizeDAG.cpp
4141:    Results.push_back(ExpandFPLibCall(Node, RTLIB::POWI_F32, RTLIB::POWI_F64,
