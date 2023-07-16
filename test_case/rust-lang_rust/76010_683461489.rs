
2020-08-30T19:30:36.9328530Z failures:
2020-08-30T19:30:36.9328670Z 
2020-08-30T19:30:36.9329340Z ---- [debuginfo-lldb] debuginfo/basic-types.rs stdout ----
2020-08-30T19:30:36.9329760Z NOTE: compiletest thinks it is using LLDB version 1103
2020-08-30T19:30:36.9330140Z NOTE: compiletest thinks it is using LLDB without native rust support
2020-08-30T19:30:36.9330390Z 
2020-08-30T19:30:36.9330640Z error: line not found in debugger output: [...]$0 = false
2020-08-30T19:30:36.9331050Z status: exit code: 0
2020-08-30T19:30:36.9332740Z command: "/usr/bin/python3" "/Users/runner/work/1/s/src/etc/lldb_batchmode.py" "/Users/runner/work/1/s/build/x86_64-apple-darwin/test/debuginfo/basic-types.lldb/a" "/Users/runner/work/1/s/build/x86_64-apple-darwin/test/debuginfo/basic-types.lldb/basic-types.debugger.script"
2020-08-30T19:30:36.9333450Z stdout:
2020-08-30T19:30:36.9334100Z ------------------------------------------
2020-08-30T19:30:36.9334300Z 
2020-08-30T19:30:36.9335210Z Hit breakpoint 1.1: where = a`basic_types::main::hf74228029bb5eac9 + 111 at basic-types.rs:116:5, address = 0x0000000100000aaf, resolved, hit count = 1 
2020-08-30T19:30:36.9336040Z LLDB batch-mode script
2020-08-30T19:30:36.9336590Z ----------------------
2020-08-30T19:30:36.9337440Z Debugger commands script is '/Users/runner/work/1/s/build/x86_64-apple-darwin/test/debuginfo/basic-types.lldb/basic-types.debugger.script'.
2020-08-30T19:30:36.9338970Z Target executable is '/Users/runner/work/1/s/build/x86_64-apple-darwin/test/debuginfo/basic-types.lldb/a'.
2020-08-30T19:30:36.9339830Z Current working directory is '/Users/runner/work/1/s'
2020-08-30T19:30:36.9340650Z Creating a target for '/Users/runner/work/1/s/build/x86_64-apple-darwin/test/debuginfo/basic-types.lldb/a'
2020-08-30T19:30:36.9341380Z settings set auto-confirm true
2020-08-30T19:30:36.9341570Z 
2020-08-30T19:30:36.9341730Z version
2020-08-30T19:30:36.9342410Z lldb-1103.0.22.10 Apple Swift version 5.2.4 (swiftlang-1103.0.32.9 clang-1103.0.32.53) 
2020-08-30T19:30:36.9342870Z command script import /Users/runner/work/1/s/./src/etc/lldb_lookup.py
2020-08-30T19:30:36.9343610Z type synthetic add -l lldb_lookup.synthetic_lookup -x '.*' --category Rust
2020-08-30T19:30:36.9344930Z type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(alloc::([a-z_]+::)+)String$' --category Rust
2020-08-30T19:30:36.9345840Z type summary add -F lldb_lookup.summary_lookup  -e -x -h '^&str$' --category Rust
2020-08-30T19:30:36.9346660Z type summary add -F lldb_lookup.summary_lookup  -e -x -h '^&\[.+\]$' --category Rust
2020-08-30T19:30:36.9347590Z type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(std::ffi::([a-z_]+::)+)OsString$' --category Rust
2020-08-30T19:30:36.9348560Z type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(alloc::([a-z_]+::)+)Vec<.+>$' --category Rust
2020-08-30T19:30:36.9349550Z type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(alloc::([a-z_]+::)+)VecDeque<.+>$' --category Rust
2020-08-30T19:30:36.9350550Z type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(alloc::([a-z_]+::)+)BTreeSet<.+>$' --category Rust
2020-08-30T19:30:36.9351850Z type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(alloc::([a-z_]+::)+)BTreeMap<.+>$' --category Rust
2020-08-30T19:30:36.9353000Z type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(std::collections::([a-z_]+::)+)HashMap<.+>$' --category Rust
2020-08-30T19:30:36.9354050Z type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(std::collections::([a-z_]+::)+)HashSet<.+>$' --category Rust
2020-08-30T19:30:36.9355070Z type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(alloc::([a-z_]+::)+)Rc<.+>$' --category Rust
2020-08-30T19:30:36.9356040Z type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(alloc::([a-z_]+::)+)Arc<.+>$' --category Rust
2020-08-30T19:30:36.9357000Z type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(core::([a-z_]+::)+)Cell<.+>$' --category Rust
2020-08-30T19:30:36.9357980Z type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(core::([a-z_]+::)+)Ref<.+>$' --category Rust
2020-08-30T19:30:36.9358940Z type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(core::([a-z_]+::)+)RefMut<.+>$' --category Rust
2020-08-30T19:30:36.9359950Z type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(core::([a-z_]+::)+)RefCell<.+>$' --category Rust
2020-08-30T19:30:36.9360480Z type category enable Rust
2020-08-30T19:30:36.9360640Z 
2020-08-30T19:30:36.9361240Z breakpoint set --file 'basic-types.rs' --line 116
2020-08-30T19:30:36.9362150Z Breakpoint 1: where = a`basic_types::main::hf74228029bb5eac9 + 111 at basic-types.rs:116:5, address = 0x0000000100000aaf 
2020-08-30T19:30:36.9362590Z run
2020-08-30T19:30:36.9364330Z Process 64688 stopped * thread #1, queue = 'com.apple.main-thread', stop reason = breakpoint 1.1 frame #0: 0x0000000100000aaf a`basic_types::main::hf74228029bb5eac9 at basic-types.rs:116:5 113 let u64: u64 = 64; 114 let f32: f32 = 2.5; 115 let f64: f64 = 3.5; -> 116 _zzz(); // #break ^ 117 } 118 119 fn _zzz() {()} Target 0: (a) stopped. Process 64688 launched: '/Users/runner/work/1/s/build/x86_64-apple-darwin/test/debuginfo/basic-types.lldb/a' (x86_64) 
2020-08-30T19:30:36.9365450Z print b
2020-08-30T19:30:36.9366190Z error: Couldn't materialize: couldn't get the value of b: extracting data from value failed
2020-08-30T19:30:36.9367020Z error: errored out in DoExecute, couldn't PrepareToExecuteJITExpression
2020-08-30T19:30:36.9367550Z 
2020-08-30T19:30:36.9367700Z print i
2020-08-30T19:30:36.9368390Z error: <user expression 1>:1:1: use of undeclared identifier 'i'
2020-08-30T19:30:36.9368680Z i
2020-08-30T19:30:36.9368840Z ^
2020-08-30T19:30:36.9368940Z 
2020-08-30T19:30:36.9369100Z print i8
2020-08-30T19:30:36.9369730Z error: <user expression 2>:1:1: use of undeclared identifier 'i8'
2020-08-30T19:30:36.9370040Z i8
2020-08-30T19:30:36.9370180Z ^
2020-08-30T19:30:36.9370300Z 
2020-08-30T19:30:36.9370450Z print i16
2020-08-30T19:30:36.9371100Z error: <user expression 3>:1:1: use of undeclared identifier 'i16'
2020-08-30T19:30:36.9371400Z i16
2020-08-30T19:30:36.9371550Z ^
2020-08-30T19:30:36.9371660Z 
2020-08-30T19:30:36.9371810Z print i32
2020-08-30T19:30:36.9372460Z error: <user expression 4>:1:1: use of undeclared identifier 'i32'
2020-08-30T19:30:36.9372980Z i32
2020-08-30T19:30:36.9373130Z ^
2020-08-30T19:30:36.9373250Z 
2020-08-30T19:30:36.9373400Z print i64
2020-08-30T19:30:36.9374100Z error: <user expression 5>:1:1: use of undeclared identifier 'i64'
2020-08-30T19:30:36.9374400Z i64
2020-08-30T19:30:36.9374550Z ^
2020-08-30T19:30:36.9374660Z 
2020-08-30T19:30:36.9374810Z print u
2020-08-30T19:30:36.9375440Z error: <user expression 6>:1:1: use of undeclared identifier 'u'
2020-08-30T19:30:36.9375750Z u
2020-08-30T19:30:36.9375890Z ^
2020-08-30T19:30:36.9375990Z 
2020-08-30T19:30:36.9376150Z print u8
2020-08-30T19:30:36.9376780Z error: <user expression 7>:1:1: use of undeclared identifier 'u8'
2020-08-30T19:30:36.9377090Z u8
2020-08-30T19:30:36.9377240Z ^
2020-08-30T19:30:36.9377350Z 
2020-08-30T19:30:36.9377490Z print u16
2020-08-30T19:30:36.9378150Z error: <user expression 8>:1:1: use of undeclared identifier 'u16'
2020-08-30T19:30:36.9378460Z u16
2020-08-30T19:30:36.9378600Z ^
2020-08-30T19:30:36.9378710Z 
2020-08-30T19:30:36.9378870Z print u32
2020-08-30T19:30:36.9379510Z error: <user expression 9>:1:1: use of undeclared identifier 'u32'
2020-08-30T19:30:36.9379830Z u32
2020-08-30T19:30:36.9379980Z ^
2020-08-30T19:30:36.9380090Z 
2020-08-30T19:30:36.9380230Z print u64
2020-08-30T19:30:36.9380890Z error: <user expression 10>:1:1: use of undeclared identifier 'u64'
2020-08-30T19:30:36.9381190Z u64
2020-08-30T19:30:36.9381340Z ^
2020-08-30T19:30:36.9381450Z 
2020-08-30T19:30:36.9381610Z print f32
2020-08-30T19:30:36.9382250Z error: <user expression 11>:1:1: use of undeclared identifier 'f32'
2020-08-30T19:30:36.9382560Z f32
2020-08-30T19:30:36.9382700Z ^
2020-08-30T19:30:36.9382820Z 
2020-08-30T19:30:36.9382960Z print f64
2020-08-30T19:30:36.9383610Z error: <user expression 12>:1:1: use of undeclared identifier 'f64'
2020-08-30T19:30:36.9383910Z f64
2020-08-30T19:30:36.9384060Z ^
2020-08-30T19:30:36.9384170Z 
2020-08-30T19:30:36.9384320Z quit
2020-08-30T19:30:36.9384440Z 
2020-08-30T19:30:36.9384550Z 
2020-08-30T19:30:36.9385100Z ------------------------------------------
2020-08-30T19:30:36.9385350Z stderr:
2020-08-30T19:30:36.9385920Z ------------------------------------------
2020-08-30T19:30:36.9386690Z error: need to add support for DW_TAG_base_type 'char' encoded with DW_ATE = 0x8, bit_size = 32
2020-08-30T19:30:36.9387490Z /Users/runner/work/1/s/src/etc/lldb_batchmode.py:148: DeprecationWarning: time.clock has been deprecated in Python 3.3 and will be removed from Python 3.8: use time.perf_counter or time.process_time instead
2020-08-30T19:30:36.9388130Z   watchdog_start_time = clock()
2020-08-30T19:30:36.9388750Z /Users/runner/work/1/s/src/etc/lldb_batchmode.py:152: DeprecationWarning: time.clock has been deprecated in Python 3.3 and will be removed from Python 3.8: use time.perf_counter or time.process_time instead
2020-08-30T19:30:36.9389410Z   while clock() < watchdog_max_time:
2020-08-30T19:30:36.9389600Z 
2020-08-30T19:30:36.9390220Z ------------------------------------------
2020-08-30T19:30:36.9390440Z 
2020-08-30T19:30:36.9390560Z 
2020-08-30T19:30:36.9390670Z 
2020-08-30T19:30:36.9390820Z failures:
2020-08-30T19:30:36.9391420Z     [debuginfo-lldb] debuginfo/basic-types.rs
2020-08-30T19:30:36.9391840Z 
2020-08-30T19:30:36.9392590Z test result: [31mFAILED(B[m. 89 passed; 1 failed; 26 ignored; 0 measured; 0 filtered out
2020-08-30T19:30:36.9392900Z 
2020-08-30T19:30:36.9393650Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:353:22
2020-08-30T19:30:36.9394120Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-08-30T19:30:36.9394390Z 
2020-08-30T19:30:36.9394500Z 
2020-08-30T19:30:36.9403710Z command did not execute successfully: "/Users/runner/work/1/s/build/x86_64-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/runner/work/1/s/build/x86_64-apple-darwin/stage2/lib" "--run-lib-path" "/Users/runner/work/1/s/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "--rustc-path" "/Users/runner/work/1/s/build/x86_64-apple-darwin/stage2/bin/rustc" "--src-base" "/Users/runner/work/1/s/src/test/debuginfo" "--build-base" "/Users/runner/work/1/s/build/x86_64-apple-darwin/test/debuginfo" "--stage-id" "stage2-x86_64-apple-darwin" "--mode" "debuginfo" "--target" "x86_64-apple-darwin" "--host" "x86_64-apple-darwin" "--llvm-filecheck" "/Users/runner/work/1/s/build/x86_64-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/usr/local/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/runner/work/1/s/build/x86_64-apple-darwin/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/runner/work/1/s/build/x86_64-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/opt/python@2/bin/python2.7" "--lldb-python" "/usr/bin/python3" "--lldb-version" "lldb-1103.0.22.10\nApple Swift version 5.2.4 (swiftlang-1103.0.32.9 clang-1103.0.32.53)\n" "--lldb-python-dir" "/Applications/Xcode_11.6.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python3" "--llvm-version" "11.0.0-rust-1.48.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions frontendopenmp fuzzmutate globalisel gtest gtest_main hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target testingsupport textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-08-30T19:30:36.9410530Z expected success, got: exit code: 101
2020-08-30T19:30:36.9410750Z 
2020-08-30T19:30:36.9410860Z 
2020-08-30T19:30:36.9411850Z failed to run: /Users/runner/work/1/s/build/bootstrap/debug/bootstrap --stage 2 test
2020-08-30T19:30:36.9412520Z Build completed unsuccessfully in 1:13:06
2020-08-30T19:30:36.9412840Z == clock drift check ==
2020-08-30T19:30:36.9452220Z   local time: Sun Aug 30 19:30:36 UTC 2020
2020-08-30T19:30:37.0440220Z   network time: Sun, 30 Aug 2020 19:30:37 GMT
2020-08-30T19:30:37.0441500Z == end clock drift check ==
2020-08-30T19:30:37.0484760Z 
2020-08-30T19:30:37.0560240Z ##[error]Bash exited with code '1'.
2020-08-30T19:30:37.0576840Z ##[section]Finishing: Run build
