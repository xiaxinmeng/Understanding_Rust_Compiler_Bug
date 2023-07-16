plain
2019-07-28T00:55:00.3197870Z status: exit code: 2
2019-07-28T00:55:00.3198520Z command: "make"
2019-07-28T00:55:00.3198720Z stdout:
2019-07-28T00:55:00.3200050Z ------------------------------------------
2019-07-28T00:55:00.3200240Z /bin/echo || exit 0 # This test requires /bin/echo to exist
2019-07-28T00:55:00.3200310Z 
2019-07-28T00:55:00.3201810Z DYLD_LIBRARY_PATH="/Users/vsts/agent/2.154.3/work/1/s/build/i686-apple-darwin/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend:/Users/vsts/agent/2.154.3/work/1/s/build/i686-apple-darwin/stage2/lib:" '/Users/vsts/agent/2.154.3/work/1/s/build/i686-apple-darwin/stage2/bin/rustc' --out-dir /Users/vsts/agent/2.154.3/work/1/s/build/i686-apple-darwin/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend -L /Users/vsts/agent/2.154.3/work/1/s/build/i686-apple-darwin/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend  the_backend.rs --crate-name the_backend --crate-type dylib \
2019-07-28T00:55:00.3202830Z   -o /Users/vsts/agent/2.154.3/work/1/s/build/i686-apple-darwin/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend/the_backend.dylib
2019-07-28T00:55:00.3204570Z DYLD_LIBRARY_PATH="/Users/vsts/agent/2.154.3/work/1/s/build/i686-apple-darwin/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend:/Users/vsts/agent/2.154.3/work/1/s/build/i686-apple-darwin/stage2/lib:" '/Users/vsts/agent/2.154.3/work/1/s/build/i686-apple-darwin/stage2/bin/rustc' --out-dir /Users/vsts/agent/2.154.3/work/1/s/build/i686-apple-darwin/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend -L /Users/vsts/agent/2.154.3/work/1/s/build/i686-apple-darwin/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend  some_crate.rs --crate-name some_crate --crate-type lib -o /Users/vsts/agent/2.154.3/work/1/s/build/i686-apple-darwin/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend/some_crate \
2019-07-28T00:55:00.3205660Z   -Z codegen-backend=/Users/vsts/agent/2.154.3/work/1/s/build/i686-apple-darwin/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend/the_backend.dylib -Z unstable-options
2019-07-28T00:55:00.3206760Z ------------------------------------------
2019-07-28T00:55:00.3206870Z stderr:
2019-07-28T00:55:00.3207490Z ------------------------------------------
2019-07-28T00:55:00.3207650Z warning: trait objects without an explicit `dyn` are deprecated
2019-07-28T00:55:00.3207650Z warning: trait objects without an explicit `dyn` are deprecated
2019-07-28T00:55:00.3208300Z   --> the_backend.rs:43:38
2019-07-28T00:55:00.3208390Z    |
2019-07-28T00:55:00.3209260Z 43 |     fn metadata_loader(&self) -> Box<MetadataLoader + Sync> {
2019-07-28T00:55:00.3209430Z    |                                      ^^^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn MetadataLoader + Sync`
2019-07-28T00:55:00.3209630Z    = note: `#[warn(bare_trait_objects)]` on by default
2019-07-28T00:55:00.3209700Z 
2019-07-28T00:55:00.3209790Z warning: trait objects without an explicit `dyn` are deprecated
2019-07-28T00:55:00.3210470Z   --> the_backend.rs:66:33
2019-07-28T00:55:00.3210470Z   --> the_backend.rs:66:33
2019-07-28T00:55:00.3210570Z    |
2019-07-28T00:55:00.3210660Z 66 |         _rx: mpsc::Receiver<Box<Any + Send>>
2019-07-28T00:55:00.3211010Z    |                                 ^^^^^^^^^^ help: use `dyn`: `dyn Any + Send`
2019-07-28T00:55:00.3211180Z warning: trait objects without an explicit `dyn` are deprecated
2019-07-28T00:55:00.3211820Z   --> the_backend.rs:67:14
2019-07-28T00:55:00.3211920Z    |
2019-07-28T00:55:00.3212510Z 67 |     ) -> Box<Any> {
2019-07-28T00:55:00.3212510Z 67 |     ) -> Box<Any> {
2019-07-28T00:55:00.3212630Z    |              ^^^ help: use `dyn`: `dyn Any`
2019-07-28T00:55:00.3212680Z 
2019-07-28T00:55:00.3212780Z warning: trait objects without an explicit `dyn` are deprecated
2019-07-28T00:55:00.3213380Z   --> the_backend.rs:75:30
2019-07-28T00:55:00.3213500Z    |
2019-07-28T00:55:00.3213570Z 75 |         ongoing_codegen: Box<Any>,
2019-07-28T00:55:00.3213670Z    |                              ^^^ help: use `dyn`: `dyn Any`
2019-07-28T00:55:00.3213840Z warning: trait objects without an explicit `dyn` are deprecated
2019-07-28T00:55:00.3214470Z    --> the_backend.rs:100:41
2019-07-28T00:55:00.3214590Z     |
2019-07-28T00:55:00.3214590Z     |
2019-07-28T00:55:00.3215210Z 100 | pub fn __rustc_codegen_backend() -> Box<CodegenBackend> {
2019-07-28T00:55:00.3215370Z     |                                         ^^^^^^^^^^^^^^ help: use `dyn`: `dyn CodegenBackend`
2019-07-28T00:55:00.3215440Z 
2019-07-28T00:55:00.3216060Z warning: ignoring --out-dir flag due to -o flag
2019-07-28T00:55:00.3216130Z 
2019-07-28T00:55:00.3216750Z warning: ignoring --out-dir flag due to -o flag
2019-07-28T00:55:00.3216840Z 
2019-07-28T00:55:00.3217650Z thread '<unnamed>' panicked at 'cannot access a scoped thread local variable without calling `set` first', /Users/vsts/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:168:9
2019-07-28T00:55:00.3217910Z 
2019-07-28T00:55:00.3218020Z error: internal compiler error: unexpected panic
2019-07-28T00:55:00.3218070Z 
2019-07-28T00:55:00.3218070Z 
2019-07-28T00:55:00.3218160Z note: the compiler unexpectedly panicked. this is a bug.
2019-07-28T00:55:00.3218220Z 
2019-07-28T00:55:00.3218920Z note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
2019-07-28T00:55:00.3219040Z 
2019-07-28T00:55:00.3219710Z note: rustc 1.38.0-dev running on i686-apple-darwin
2019-07-28T00:55:00.3219840Z 
2019-07-28T00:55:00.3220820Z note: compiler flags: -Z codegen-backend=/Users/vsts/agent/2.154.3/work/1/s/build/i686-apple-darwin/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend/the_backend.dylib -Z unstable-options --crate-type lib
2019-07-28T00:55:00.3220970Z 
2019-07-28T00:55:00.3221060Z make: *** [all] Error 101
2019-07-28T00:55:00.3221740Z ------------------------------------------
2019-07-28T00:55:00.3221810Z 
2019-07-28T00:55:00.3221870Z 
2019-07-28T00:55:00.3221920Z 
---
2019-07-28T00:55:00.3224430Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:534:22
2019-07-28T00:55:00.3224580Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-28T00:55:00.3224840Z 
2019-07-28T00:55:00.3224950Z 
2019-07-28T00:55:00.3231360Z command did not execute successfully: "/Users/vsts/agent/2.154.3/work/1/s/build/i686-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/vsts/agent/2.154.3/work/1/s/build/i686-apple-darwin/stage2/lib" "--run-lib-path" "/Users/vsts/agent/2.154.3/work/1/s/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib" "--rustc-path" "/Users/vsts/agent/2.154.3/work/1/s/build/i686-apple-darwin/stage2/bin/rustc" "--rustdoc-path" "/Users/vsts/agent/2.154.3/work/1/s/build/i686-apple-darwin/stage2/bin/rustdoc" "--src-base" "/Users/vsts/agent/2.154.3/work/1/s/src/test/run-make-fulldeps" "--build-base" "/Users/vsts/agent/2.154.3/work/1/s/build/i686-apple-darwin/test/run-make-fulldeps" "--stage-id" "stage2-i686-apple-darwin" "--mode" "run-make" "--target" "i686-apple-darwin" "--host" "i686-apple-darwin" "--llvm-filecheck" "/Users/vsts/agent/2.154.3/work/1/s/build/i686-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/usr/local/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/vsts/agent/2.154.3/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/vsts/agent/2.154.3/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/bin/python2.7" "--lldb-python" "/usr/bin/python" "--lldb-version" "lldb-902.0.79.2\n  Swift-4.1\n" "--lldb-python-dir" "/Applications/Xcode_9.3.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" "--llvm-version" "9.0.0-rust-1.38.0-dev\n" "--cc" "/Users/vsts/agent/2.154.3/work/1/s/clang+llvm-7.0.0-x86_64-apple-darwin/bin/clang" "--cxx" "/Users/vsts/agent/2.154.3/work/1/s/clang+llvm-7.0.0-x86_64-apple-darwin/bin/clang++" "--cflags" "-ffunction-sections -fdata-sections -fPIC --target=i686-apple-darwin -stdlib=libc++" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter binaryformat bitreader bitstreamreader bitwriter codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel gtest gtest_main hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target testingsupport textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xray" "--llvm-cxxflags" "-I/Users/vsts/agent/2.154.3/work/1/s/src/llvm-project/llvm/include -I/Users/vsts/agent/2.154.3/work/1/s/build/i686-apple-darwin/llvm/build/include -std=c++11 -stdlib=libc++  -fno-exceptions -fno-rtti -D_LARGEFILE_SOURCE -D_FILE_OFFSET_BITS=64 -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--llvm-bin-dir" "/Users/vsts/agent/2.154.3/work/1/s/build/i686-apple-darwin/llvm/build/bin" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-07-28T00:55:00.3233960Z 
2019-07-28T00:55:00.3234010Z 
2019-07-28T00:55:00.3234160Z failed to run: /Users/vsts/agent/2.154.3/work/1/s/build/bootstrap/debug/bootstrap test
2019-07-28T00:55:00.3234260Z Build completed unsuccessfully in 1:47:45
2019-07-28T00:55:00.3234260Z Build completed unsuccessfully in 1:47:45
2019-07-28T00:55:00.3496930Z ##[error]Bash exited with code '1'.
2019-07-28T00:55:00.3558130Z ##[section]Starting: Upload CPU usage statistics
2019-07-28T00:55:00.3583280Z ==============================================================================
2019-07-28T00:55:00.3583410Z Task         : Bash
2019-07-28T00:55:00.3583490Z Description  : Run a Bash script on macOS, Linux, or Windows
