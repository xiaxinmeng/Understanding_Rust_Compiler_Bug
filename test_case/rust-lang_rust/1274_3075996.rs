
Optional Features:
  --disable-FEATURE       do not include FEATURE (same as --enable-FEATURE=no)
  --enable-FEATURE[=ARG]  include FEATURE [ARG=yes]
  --enable-polly          Use polly if available (default is YES)
  --enable-libcpp         Use libc++ if available (default is NO)
  --enable-optimized      Compile with optimizations enabled (default is NO)
  --enable-profiling      Compile with profiling enabled (default is NO)
  --enable-assertions     Compile with assertion checks enabled (default is
                          YES)
  --enable-expensive-checks
                          Compile with expensive debug checks enabled (default
                          is NO)
  --enable-debug-runtime  Build runtime libs with debug symbols (default is
                          NO)
  --enable-debug-symbols  Build compiler with debug symbols (default is NO if
                          optimization is on and YES if it's off)
  --enable-jit            Enable Just In Time Compiling (default is YES)
  --enable-docs           Build documents (default is YES)
  --enable-doxygen        Build doxygen documentation (default is NO)
  --enable-threads        Use threads if available (default is YES)
  --enable-pthreads       Use pthreads if available (default is YES)
  --enable-pic            Build LLVM with Position Independent Code (default
                          is YES)
  --enable-shared         Build a shared library and link tools against it
                          (default is NO)
  --enable-embed-stdcxx   Build a shared library with embedded libstdc++ for
                          Win32 DLL (default is NO)
  --enable-timestamps     Enable embedding timestamp information in build
                          (default is YES)
  --enable-targets        Build specific host targets: all or
                          target1,target2,... Valid targets are: host, x86,
                          x86_64, sparc, powerpc, arm, mips, spu, xcore,
                          msp430, ptx, cbe, and cpp (default=all)
  --enable-cbe-printf-a   Enable C Backend output with hex floating point via
                          %a (default is YES)
  --enable-bindings       Build specific language bindings:
                          all,auto,none,{binding-name} (default=auto)
  --enable-libffi         Check for the presence of libffi (default is NO)
  --enable-ltdl-install   install libltdl
