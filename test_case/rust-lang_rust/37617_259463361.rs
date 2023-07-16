
--- stderr
usage: llvm-config <OPTION>... [<COMPONENT>...]

Get various configuration information needed to compile programs which use
LLVM.  Typically called from 'configure' scripts.  Examples:
  llvm-config --cxxflags
  llvm-config --ldflags
  llvm-config --libs engine bcreader scalaropts

Options:
  --version         Print LLVM version.
  --prefix          Print the installation prefix.
  --src-root        Print the source root LLVM was built from.
  --obj-root        Print the object root used to build LLVM.
  --bindir          Directory containing LLVM executables.
  --includedir      Directory containing LLVM headers.
  --libdir          Directory containing LLVM libraries.
  --cppflags        C preprocessor flags for files that include LLVM headers.
  --cflags          C compiler flags for files that include LLVM headers.
  --cxxflags        C++ compiler flags for files that include LLVM headers.
  --ldflags         Print Linker flags.
  --system-libs     System Libraries needed to link against LLVM components.
  --libs            Libraries needed to link against LLVM components.
  --libnames        Bare library names for in-tree builds.
  --libfiles        Fully qualified library filenames for makefile depends.
  --components      List of all possible components.
  --targets-built   List of all targets currently built.
  --host-target     Target triple used to configure LLVM.
  --build-mode      Print build mode of LLVM tree (e.g. Debug or Release).
  --assertion-mode  Print assertion mode of LLVM tree (ON or OFF).
  --build-system    Print the build system used to build LLVM (autoconf or cmake).
  --has-rtti        Print whether or not LLVM was built with rtti (YES or NO).
  --shared-mode     Print how the provided components can be collectively linked (`shared` or `static`).
Typical components:
  all               All LLVM libraries (default).
  engine            Either a native JIT or a bitcode interpreter.
thread 'main' panicked at 'command did not execute successfully: "/usr/bin/llvm-config" "--libs" "--link-static" "--system-libs" "aarch64" "arm" "asmparser" "bitreader" "bitwriter" "instrumentation" "interpreter" "ipo" "linker" "mcjit" "mips" "powerpc" "systemz" "x86"
expected success, got: exit code: 1', /shared/rust/checkouts/msp430/src/build_helper/lib.rs:69
note: Run with `RUST_BACKTRACE=1` for a backtrace.
