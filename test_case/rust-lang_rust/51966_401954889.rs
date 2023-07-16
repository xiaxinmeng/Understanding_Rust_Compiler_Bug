plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:0a5d6fa6
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/30/78/3ee2d1d05aadd79884402193c4d2967d042f29c941586ce450ce86fdc609/awscli-1.15.50-py2.py3-none-any.whl (1.3MB)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/db/c8/7dcf9dbcb22429512708fe3a547f8b6101c0d02137acbd892505aee57adf/colorama-0.3.9-py2.py3-none-any.whl
Collecting rsa<=3.5.0,>=3.1.2 (from awscli)
---
Requirement already satisfied: PyYAML<=3.12,>=3.10 in /usr/lib/python2.7/dist-packages (from awscli)
Collecting botocore==1.10.49 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/8d/10/488051d15156c325fec7347559dd4a54edc903d4cda9158f310a3e26e311/botocore-1.10.49-py2.py3-none-any.whl (4.4MB)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/50/09/c53398e0005b11f7ffb27b7aa720c617aba53be4fb4f4f3f06b9b5c60f28/docutils-0.14-py2-none-any.whl (543kB)
Collecting pyasn1>=0.1.3 (from rsa<=3.5.0,>=3.1.2->awscli)
---
[00:20:00] -- Looking for dlopen in dl
[00:20:00] -- Looking for dlopen in dl - found
[00:20:00] -- Looking for clock_gettime in rt
[00:20:00] -- Looking for clock_gettime in rt - found
[00:20:00] -- Looking for pfm_initialize in pfm
[00:20:00] -- Looking for pfm_initialize in pfm - not found
[00:20:01] -- Looking for pthread.h - found
[00:20:01] -- Looking for pthread_create
[00:20:01] -- Looking for pthread_create - not found
[00:20:01] -- Looking for pthread_create in pthreads
---
[00:21:00] [  2%] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCAsmMacro.cpp.o
[00:21:00] [  2%] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCAsmStreamer.cpp.o
[00:21:00] [  2%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/DAGDeltaAlgorithm.cpp.o
[00:21:01] [  2%] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCAssembler.cpp.o
[00:21:03] [  2%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/DJB.cpp.o
[00:21:04] [  2%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/ErrorHandling.cpp.o
[00:21:05] [  2%] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCCodeEmitter.cpp.o
[00:21:05] [  2%] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCCodePadder.cpp.o
[00:21:06] [  2%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/FileUtilities.cpp.o
---
[00:22:02] [  8%] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCWasmObjectTargetWriter.cpp.o
[00:22:02] [  8%] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCWasmStreamer.cpp.o
[00:22:03] [  8%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/DAGISelMatcherGen.cpp.o
[00:22:03] [  8%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/Unicode.cpp.o
[00:22:03] [  8%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/UnicodeCaseFold.cpp.o
[00:22:03] [  8%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/VersionTuple.cpp.o
[00:22:04] [  8%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/WithColor.cpp.o
[00:22:05] [  8%] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCWin64EH.cpp.o
[00:22:06] [  8%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/YAMLTraits.cpp.o
[00:22:07] [  8%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/DAGISelMatcherOpt.cpp.o
[00:22:07] [  8%] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCWinCOFFStreamer.cpp.o
---
[00:22:28] [ 10%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/GlobalISelEmitter.cpp.o
[00:22:29] [ 10%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/Program.cpp.o
[00:22:30] [ 10%] Building CXX object lib/MC/MCParser/CMakeFiles/LLVMMCParser.dir/COFFAsmParser.cpp.o
[00:22:31] In file included from /checkout/src/llvm/lib/Support/Program.cpp:79:
[00:22:31] /checkout/src/llvm/lib/Support/Unix/Program.inc:462:24: error: use of undeclared identifier '_POSIX_ARG_MAX'; did you mean '_SC_ARG_MAX'?
[00:22:31]   static long ArgMin = _POSIX_ARG_MAX;
[00:22:31]                        ^~~~~~~~~~~~~~
[00:22:31]                        _SC_ARG_MAX
[00:22:31] /usr/include/bits/confname.h:75:5: note: '_SC_ARG_MAX' declared here
[00:22:31]     _SC_ARG_MAX,
[00:22:31] 1 error generated.
[00:22:31] 1 error generated.
[00:22:31] gmake[2]: *** [lib/Support/CMakeFiles/LLVMSupport.dir/Program.cpp.o] Error 1
[00:22:31] gmake[1]: *** [lib/Support/CMakeFiles/LLVMSupport.dir/all] Error 2
[00:22:31] gmake[1]: *** Waiting for unfinished jobs....
[00:22:33] [ 10%] Building CXX object lib/MC/MCParser/CMakeFiles/LLVMMCParser.dir/ELFAsmParser.cpp.o
[00:22:35] [ 10%] Building CXX object lib/MC/MCParser/CMakeFiles/LLVMMCParser.dir/MCAsmLexer.cpp.o
[00:22:35] [ 10%] Building CXX object lib/MC/MCParser/CMakeFiles/LLVMMCParser.dir/MCAsmParser.cpp.o
[00:22:36] [ 10%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/InfoByHwMode.cpp.o
---
[00:22:40] [ 10%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/IntrinsicEmitter.cpp.o
[00:22:43] [ 10%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/OptParserEmitter.cpp.o
[00:22:45] [ 10%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/PredicateExpander.cpp.o
[00:22:45] [ 10%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/PseudoLoweringEmitter.cpp.o
[00:22:47] [ 10%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/RISCVCompressInstEmitter.cpp.o
[00:22:49] [ 11%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/RegisterInfoEmitter.cpp.o
[00:22:52] [ 11%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/SDNodeProperties.cpp.o
[00:22:52] [ 11%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/SearchableTableEmitter.cpp.o
[00:22:53] [ 11%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/SubtargetEmitter.cpp.o
---
[00:23:03] [ 11%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/X86RecognizableInstr.cpp.o
[00:23:03] [ 11%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/WebAssemblyDisassemblerEmitter.cpp.o
[00:23:05] [ 11%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/CTagsEmitter.cpp.o
[00:23:10] [ 11%] Built target obj.llvm-tblgen
[00:23:10] gmake: *** [all] Error 2
[00:23:10] command did not execute successfully, got: exit code: 2
[00:23:10] 
[00:23:10] 
[00:23:10] build script failed, must exit now', /cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.30/src/lib.rs:643:5
[00:23:10]  finished in 196.511
[00:23:10] travis_fold:end:llvm

[00:23:10] travis_time:end:llvm:start=1530569685893371040,finish=1530569882404891947,duration=196511520907
---
travis_time:end:00d5cc26:start=1530569883016777347,finish=1530569883023555253,duration=6777906
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1e3047b8
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:04ed012b
$ dmesg | grep -i kill
