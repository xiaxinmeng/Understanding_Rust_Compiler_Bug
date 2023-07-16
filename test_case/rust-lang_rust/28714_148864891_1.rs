 bash
$ llvm-config --cflags
-I/usr/include -O2 -pipe -march=native -ggdb -fvar-tracking-assignments -fno-omit-frame-pointer -ftrack-macro-expansion=2 -fstack-protector-all -fPIC  -fPIC -Wall -W -Wno-unused-parameter -Wwrite-strings -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-comment -Werror=date-time -ffunction-sections -fdata-sections  -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS
$ llc --version
LLVM (http://llvm.org/):
  LLVM version 3.8.0svn
  Optimized build.
  Default target: x86_64-pc-linux-gnu
  Host CPU: (unknown)

  Registered Targets:
    amdgcn - AMD GCN GPUs
    cpp    - C++ backend
    r600   - AMD GPUs HD2XXX-HD6XXX
    x86    - 32-bit X86: Pentium-Pro and above
    x86-64 - 64-bit X86: EM64T and AMD64
