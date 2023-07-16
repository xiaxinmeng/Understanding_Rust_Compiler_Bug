
[01:22:58] failures:
[01:22:58] 
[01:22:58] ---- [run-make] run-make/type-mismatch-same-crate-name stdout ----
[01:22:58] 	
[01:22:58] error: make failed
[01:22:58] status: exit code: 2
[01:22:58] command: "make"
[01:22:58] stdout:
[01:22:58] ------------------------------------------
[01:22:58] # compile two different versions of crateA
[01:22:58] DYLD_LIBRARY_PATH="/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/run-make/type-mismatch-same-crate-name.stage2-x86_64-apple-darwin:/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib:" '/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/bin/rustc' --out-dir /Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/run-make/type-mismatch-same-crate-name.stage2-x86_64-apple-darwin -L /Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/run-make/type-mismatch-same-crate-name.stage2-x86_64-apple-darwin  --crate-type=rlib crateA.rs -C metadata=-1 -C extra-filename=-1
[01:22:58] DYLD_LIBRARY_PATH="/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/run-make/type-mismatch-same-crate-name.stage2-x86_64-apple-darwin:/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib:" '/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/bin/rustc' --out-dir /Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/run-make/type-mismatch-same-crate-name.stage2-x86_64-apple-darwin -L /Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/run-make/type-mismatch-same-crate-name.stage2-x86_64-apple-darwin  --crate-type=rlib crateA.rs -C metadata=-2 -C extra-filename=-2
[01:22:58] # make crateB depend on version 1 of crateA
[01:22:58] DYLD_LIBRARY_PATH="/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/run-make/type-mismatch-same-crate-name.stage2-x86_64-apple-darwin:/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib:" '/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/bin/rustc' --out-dir /Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/run-make/type-mismatch-same-crate-name.stage2-x86_64-apple-darwin -L /Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/run-make/type-mismatch-same-crate-name.stage2-x86_64-apple-darwin  --crate-type=rlib crateB.rs --extern crateA=/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/run-make/type-mismatch-same-crate-name.stage2-x86_64-apple-darwin/libcrateA-1.rlib
[01:22:58] # make crateC depend on version 2 of crateA
[01:22:58] DYLD_LIBRARY_PATH="/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/run-make/type-mismatch-same-crate-name.stage2-x86_64-apple-darwin:/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib:" '/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/bin/rustc' --out-dir /Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/run-make/type-mismatch-same-crate-name.stage2-x86_64-apple-darwin -L /Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/run-make/type-mismatch-same-crate-name.stage2-x86_64-apple-darwin  crateC.rs --extern crateA=/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/run-make/type-mismatch-same-crate-name.stage2-x86_64-apple-darwin/libcrateA-2.rlib 2>&1 | \
[01:22:58] 		grep -z \
[01:22:58] 	"mismatched types.*\
[01:22:58] 	crateB::try_foo(foo2);.*\
[01:22:58] 	expected struct \`crateA::foo::Foo\`, found struct \`crateA::Foo\`.*\
[01:22:58] 	different versions of crate \`crateA\`.*\
[01:22:58] 	mismatched types.*\
[01:22:58] 	crateB::try_bar(bar2);.*\
[01:22:58] 	expected trait \`crateA::bar::Bar\`, found trait \`crateA::Bar\`.*\
[01:22:58] 	different versions of crate \`crateA\`"
[01:22:58] 
[01:22:58] ------------------------------------------
[01:22:58] stderr:
[01:22:58] ------------------------------------------
[01:22:58] warning: unused variable: `x`
[01:22:58]   --> crateB.rs:13:16
[01:22:58]    |
[01:22:58] 13 | pub fn try_foo(x: crateA::Foo){}
[01:22:58]    |                ^
[01:22:58]    |
[01:22:58]    = note: #[warn(unused_variables)] on by default
[01:22:58] 
[01:22:58] warning: unused variable: `x`
[01:22:58]   --> crateB.rs:14:16
[01:22:58]    |
[01:22:58] 14 | pub fn try_bar(x: Box<crateA::Bar>){}
[01:22:58]    |                ^
[01:22:58]    |
[01:22:58]    = note: #[warn(unused_variables)] on by default
[01:22:58] 
[01:22:58] grep: invalid option -- z
[01:22:58] usage: grep [-abcDEFGHhIiJLlmnOoqRSsUVvwxZ] [-A num] [-B num] [-C[num]]
[01:22:58] 	[-e pattern] [-f file] [--binary-files=value] [--color=when]
[01:22:58] 	[--context[=num]] [--directories=action] [--label] [--line-buffered]
[01:22:58] 	[--null] [pattern] [file ...]
[01:22:58] make[1]: *** [all] Error 2
[01:22:58] 
[01:22:58] ------------------------------------------
[01:22:58] 
[01:22:58] thread '[run-make] run-make/type-mismatch-same-crate-name' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2479
[01:22:58] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:22:58] 
[01:22:58] 
[01:22:58] failures:
[01:22:58]     [run-make] run-make/type-mismatch-same-crate-name
[01:22:58] 
[01:22:58] test result: FAILED. 152 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:22:58] 
[01:22:58] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:325
[01:22:58] 
[01:22:58] 
[01:22:58] command did not execute successfully: "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage0-tools/x86_64-apple-darwin/release/compiletest" "--compile-lib-path" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib" "--run-lib-path" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "--rustc-path" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/bin/rustc" "--rustdoc-path" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/bin/rustdoc" "--src-base" "/Users/travis/build/rust-lang/rust/src/test/run-make" "--build-base" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/run-make" "--stage-id" "stage2-x86_64-apple-darwin" "--mode" "run-make" "--target" "x86_64-apple-darwin" "--host" "x86_64-apple-darwin" "--llvm-filecheck" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/Users/travis/.nvm/versions/node/v6.9.1/bin/node" "--host-rustcflags" "-Crpath -O" "--target-rustcflags" "-Crpath -O -Lnative=/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/bin/python2.7" "--lldb-python" "/usr/bin/python" "--lldb-version" "lldb-360.1.70" "--lldb-python-dir" "/Applications/Xcode.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" "--llvm-version" "4.0.1\n" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64 -stdlib=libc++" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils all all-targets analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo asmparser asmprinter bitreader bitwriter codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle engine executionengine globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jsbackend jsbackendcodegen jsbackenddesc jsbackendinfo libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes pnacltransforms powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils" "--llvm-cxxflags" "-I/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/llvm/include -ffunction-sections -fdata-sections -fPIC -m64 -stdlib=libc++ -fPIC -fvisibility-inlines-hidden -Wall -W -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wmissing-field-initializers -pedantic -Wno-long-long -Wcovered-switch-default -Wnon-virtual-dtor -Wdelete-non-virtual-dtor -Wstring-conversion -Werror=date-time -std=c++11 -O3   -fno-exceptions -fno-rtti -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:22:58] expected success, got: exit code: 101
