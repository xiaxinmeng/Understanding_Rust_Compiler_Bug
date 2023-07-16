plain
      Memory: 14 GB
      Boot ROM Version: VMW71.00V.13989454.B64.1906190538
      Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
      SMC Version (system): 2.8f0
      Serial Number (system): VMWaQzzNYyqS

hw.ncpu: 3
hw.byteorder: 1234
hw.memsize: 15032385536
---
Some tests failed in compiletest suite=debuginfo mode=debuginfo host=x86_64-apple-darwin target=x86_64-apple-darwin

failures:

---- [debuginfo-lldb] debuginfo/pretty-std-collections.rs stdout ----
NOTE: compiletest thinks it is using LLDB version 1200
NOTE: compiletest thinks it is using LLDB without native rust support
error: Error while running LLDB
status: exit code: 1
status: exit code: 1
command: "/usr/bin/python3" "/Users/runner/work/rust/rust/src/etc/lldb_batchmode.py" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/pretty-std-collections.lldb/a" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/pretty-std-collections.lldb/pretty-std-collections.debugger.script"
------------------------------------------
------------------------------------------
LLDB batch-mode script
----------------------
Debugger commands script is '/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/pretty-std-collections.lldb/pretty-std-collections.debugger.script'.
Target executable is '/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/pretty-std-collections.lldb/a'.
Current working directory is '/Users/runner/work/rust/rust'
Creating a target for '/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/pretty-std-collections.lldb/a'
settings set auto-confirm true
version
version
lldb-1200.0.41 Apple Swift version 5.3.1 (swiftlang-1200.0.41 clang-1200.0.32.8) 
command script import /Users/runner/work/rust/rust/./src/etc/lldb_lookup.py
type synthetic add -l lldb_lookup.synthetic_lookup -x '.*' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(alloc::([a-z_]+::)+)String$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^&str$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^&\[.+\]$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(std::ffi::([a-z_]+::)+)OsString$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(alloc::([a-z_]+::)+)Vec<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(alloc::([a-z_]+::)+)VecDeque<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(alloc::([a-z_]+::)+)BTreeSet<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(alloc::([a-z_]+::)+)BTreeMap<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(std::collections::([a-z_]+::)+)HashMap<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(std::collections::([a-z_]+::)+)HashSet<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(alloc::([a-z_]+::)+)Rc<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(alloc::([a-z_]+::)+)Arc<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(core::([a-z_]+::)+)Cell<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(core::([a-z_]+::)+)Ref<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(core::([a-z_]+::)+)RefMut<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(core::([a-z_]+::)+)RefCell<.+>$' --category Rust
type category enable Rust

breakpoint set --file 'pretty-std-collections.rs' --line 158
DEBUG: breakpoint added, id = 1
Breakpoint 1: where = a`pretty_std_collections::main::h1ef1d7ce5967b6a7 + 1513 at pretty-std-collections.rs:158:5, address = 0x000000010003abb9 
DEBUG: registering breakpoint callback, id = 1
Error while trying to register breakpoint callback, id = 1, message = error: could not get num args: can't find callable: breakpoint_callback
run
run
Process 64532 stopped * thread #1, queue = 'com.apple.main-thread', stop reason = breakpoint 1.1 frame #0: 0x000000010003abb9 a`pretty_std_collections::main::h1ef1d7ce5967b6a7 at pretty-std-collections.rs:158:5 155 hash_set.insert(i); 156 } 157 -> 158 zzz(); // #break ^ 159 } 160 161 fn zzz() { Target 0: (a) stopped. Process 64532 launched: '/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/pretty-std-collections.lldb/a' (x86_64) 
print vec_deque
(alloc::collections::vec_deque::VecDeque<int>) $0 = size=3 { [0] = 5 [1] = 3 [2] = 7 } 
print vec_deque2
(alloc::collections::vec_deque::VecDeque<int>) $1 = size=7 { [0] = 2 [1] = 3 [2] = 4 [3] = 5 [4] = 6 [5] = 7 [6] = 8 } 
print hash_map
------------------------------------------
stderr:
------------------------------------------
------------------------------------------
error: need to add support for DW_TAG_base_type '()' encoded with DW_ATE = 0x7, bit_size = 0
clang: CommandLine Error: Option 'h' registered more than once!
LLVM ERROR: inconsistency in registered CommandLine options
------------------------------------------




failures:
    [debuginfo-lldb] debuginfo/pretty-std-collections.rs

test result: FAILED. 89 passed; 1 failed; 26 ignored; 0 measured; 0 filtered out



command did not execute successfully: "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib" "--run-lib-path" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "--rustc-path" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/bin/rustc" "--src-base" "/Users/runner/work/rust/rust/src/test/debuginfo" "--build-base" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo" "--stage-id" "stage2-x86_64-apple-darwin" "--suite" "debuginfo" "--mode" "debuginfo" "--target" "x86_64-apple-darwin" "--host" "x86_64-apple-darwin" "--llvm-filecheck" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/usr/local/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/runner/work/rust/rust/build/x86_64-apple-darwin/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/runner/work/rust/rust/build/x86_64-apple-darwin/native/rust-test-helpers" "--docck-python" "/Library/Frameworks/Python.framework/Versions/2.7/Resources/Python.app/Contents/MacOS/Python" "--lldb-python" "/usr/bin/python3" "--lldb-version" "lldb-1200.0.41\nApple Swift version 5.3.1 (swiftlang-1200.0.41 clang-1200.0.32.8)\n" "--lldb-python-dir" "/Applications/Xcode_12.2.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python3" "--llvm-version" "11.0.0-rust-1.49.0-beta" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions frontendopenmp fuzzmutate globalisel gtest gtest_main hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target testingsupport textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /Users/runner/work/rust/rust/build/bootstrap/debug/bootstrap --stage 2 test
Build completed unsuccessfully in 1:37:07
