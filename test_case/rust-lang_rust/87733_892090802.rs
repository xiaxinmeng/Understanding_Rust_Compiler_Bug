
error: linking with `lld-link.exe` failed: exit code: 1
  |
  = note: "lld-link.exe" [..]
  = note: lld-link: error: undefined symbol: __declspec(dllimport) __std_init_once_begin_initialize
          >>> referenced by librustc_llvm-b2b2c2603e5831d1.rlib(X86TargetMachine.cpp.obj):(void __cdecl llvm::initializeX86ExecutionDomainFixPass(class llvm::PassRegistry &))
          >>> referenced by librustc_llvm-b2b2c2603e5831d1.rlib(ARMTargetMachine.cpp.obj):(void __cdecl llvm::initializeARMExecutionDomainFixPass(class llvm::PassRegistry &))
          >>> referenced by librustc_llvm-b2b2c2603e5831d1.rlib(X86LowerAMXType.cpp.obj):(void __cdecl llvm::initializeX86LowerAMXTypeLegacyPassPass(class llvm::PassRegistry &))
          >>> referenced 483 more times

          lld-link: error: undefined symbol: __declspec(dllimport) __std_init_once_complete
          >>> referenced by librustc_llvm-b2b2c2603e5831d1.rlib(X86TargetMachine.cpp.obj):(void __cdecl llvm::initializeX86ExecutionDomainFixPass(class llvm::PassRegistry &))
          >>> referenced by librustc_llvm-b2b2c2603e5831d1.rlib(ARMTargetMachine.cpp.obj):(void __cdecl llvm::initializeARMExecutionDomainFixPass(class llvm::PassRegistry &))
          >>> referenced by librustc_llvm-b2b2c2603e5831d1.rlib(X86LowerAMXType.cpp.obj):(void __cdecl llvm::initializeX86LowerAMXTypeLegacyPassPass(class llvm::PassRegistry &))
          >>> referenced 483 more times

          lld-link: error: undefined symbol: __std_system_error_allocate_message
          >>> referenced by librustc_llvm-b2b2c2603e5831d1.rlib(X86LoadValueInjectionLoadHardening.cpp.obj):(public: virtual class std::basic_string<char, struct std::char_traits<char>, class std::allocator<char>> __cdecl std::_System_error_category::message(int) const)
          >>> referenced by librustc_llvm-b2b2c2603e5831d1.rlib(ErrorHandling.cpp.obj)
          >>> referenced by librustc_llvm-b2b2c2603e5831d1.rlib(Error.cpp.obj)

          lld-link: error: undefined symbol: __std_system_error_deallocate_message
          >>> referenced by librustc_llvm-b2b2c2603e5831d1.rlib(X86LoadValueInjectionLoadHardening.cpp.obj):(public: virtual class std::basic_string<char, struct std::char_traits<char>, class std::allocator<char>> __cdecl std::_System_error_category::message(int) const)
          >>> referenced by librustc_llvm-b2b2c2603e5831d1.rlib(ErrorHandling.cpp.obj)
          >>> referenced by librustc_llvm-b2b2c2603e5831d1.rlib(Error.cpp.obj)

          lld-link: error: undefined symbol: __std_reverse_copy_trivially_copyable_1
          >>> referenced by librustc_llvm-b2b2c2603e5831d1.rlib(GCOVProfiling.cpp.obj):(private: bool __cdecl `anonymous namespace'::GCOVProfiler::emitProfileNotes(class llvm::NamedMDNode *, bool, class NamedMDNode::function_ref<class llvm::BlockFrequencyInfo * __cdecl(class llvm::Function &)>, class NamedMDNode::function_ref<class llvm::BranchProbabilityInfo * __cdecl(class llvm::Function &)>, class NamedMDNode::function_ref<class llvm::TargetLibraryInfo const & __cdecl(class llvm::Function &)>))
