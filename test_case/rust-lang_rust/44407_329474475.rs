
[00:07:21] cargo:warning=../rustllvm/PassWrapper.cpp: In function 'void LLVMRustPrintTargetCPUs(LLVMTargetMachineRef)':
[00:07:21] cargo:warning=../rustllvm/PassWrapper.cpp:310:55: error: cannot pass objects of non-trivially-copyable type 'std::string {aka class std::basic_string<char>}' through '...'
[00:07:21] cargo:warning=       MaxCPULen, "native", sys::getHostCPUName().str());
[00:07:21] cargo:warning=                                                       ^
[00:07:21] cargo:warning=../rustllvm/PassWrapper.cpp:310:55: warning: format '%s' expects argument of type 'char*', but argument 4 has type 'std::string {aka std::basic_string<char>}' [-Wformat=]
