plain
Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:10:52] 
[01:10:52] running 187 tests
[01:11:21] ....................................................................................................
[01:12:15] ........................................................................F.............test [run-make] run-make-fulldeps/long-linker-command-lines has been running for over 60 seconds
[01:13:07] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[01:13:07] failures:
[01:13:07] 
[01:13:07] ---- [run-make] run-make-fulldeps/target-specs stdout ----
[01:13:07] 
[01:13:07] 
[01:13:07] error: make failed
[01:13:07] status: exit code: 2
[01:13:07] command: "make"
[01:13:07] stdout:
[01:13:07] ------------------------------------------
[01:13:07] make[1]: Entering directory '/checkout/src/test/run-make-fulldeps/target-specs'
[01:13:07] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/target-specs/target-specs:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/target-specs/target-specs -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/target-specs/target-specs  foo.rs --target=my-awesome-platform.json --crate-type=lib --emit=asm
[01:13:07] "/checkout/src/etc/cat-and-grep.sh" -v morestack < /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/target-specs/target-specs/foo.s
[01:13:07] [[[ begin stdout ]]]
[01:13:07]  .text
[01:13:07]  .file "foo0-8787f43e282added376259c1adb08b80.rs"
[01:13:07]  .section .text._ZN3foo5start17h12186a10b974d6ddE,"ax",@progbits
[01:13:07]  .hidden _ZN3foo5start17h12186a10b974d6ddE
[01:13:07]  .globl _ZN3foo5start17h12186a10b974d6ddE
[01:13:07]  .p2align 4, 0x90
[01:13:07]  .type _ZN3foo5start17h12186a10b974d6ddE,@function
[01:13:07] _ZN3foo5start17h12186a10b974d6ddE:
[01:13:07]  .cfi_startproc
[01:13:07]  xorl %eax, %eax
[01:13:07]  retl
[01:13:07] .Lfunc_end0:
[01:13:07]  .size _ZN3foo5start17h12186a10b974d6ddE, .Lfunc_end0-_ZN3foo5start17h12186a10b974d6ddE
[01:13:07]  .cfi_endproc
[01:13:07] 
[01:13:07] 
[01:13:07]  .section ".note.GNU-stack","",@progbits
[01:13:07] 
[01:13:07] [[[ end stdout ]]]
[01:13:07] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/target-specs/target-specs:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/target-specs/target-specs -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/target-specs/target-specs  foo.rs --target=my-invalid-platform.json 2>&1 | "/checkout/src/etc/cat-and-grep.sh" "Error loading target specification"
[01:13:07] [[[ begin stdout ]]]
[01:13:07] error: Error loading target specification: SyntaxError(InvalidSyntax, 1, 1)
[01:13:07]   |
[01:13:07]   = help: Use `--print target-list` for a list of built-in targets
[01:13:07] 
[01:13:07] 
[01:13:07] [[[ end stdout ]]]
[01:13:07] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/target-specs/target-specs:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/target-specs/target-specs -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/target-specs/target-specs  foo.rs --target=my-incomplete-platform.json 2>&1 | "/checkout/src/etc/cat-and-grep.sh" 'Field llvm-target'
[01:13:07] [[[ begin stdout ]]]
[01:13:07] error: Error loading target specification: Field llvm-target in target specification is required
[01:13:07]   |
[01:13:07]   = help: Use `--print target-list` for a list of built-in targets
[01:13:07] 
[01:13:07] 
[01:13:07] [[[ end stdout ]]]
[01:13:07] RUST_TARGET_PATH=. LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/target-specs/target-specs:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/target-specs/target-specs -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/target-specs/target-specs  foo.rs --target=my-awesome-platform --crate-type=lib --emit=asm
[01:13:07] RUST_TARGET_PATH=. LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/target-specs/target-specs:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/target-specs/target-specs -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/target-specs/target-specs  foo.rs --target=my-x86_64-unknown-linux-gnu-platform --crate-type=lib --emit=asm
[01:13:07] Makefile:3: recipe for target 'all' failed
[01:13:07] make[1]: Leaving directory '/checkout/src/test/run-make-fulldeps/target-specs'
[01:13:07] ------------------------------------------
[01:13:07] stderr:
[01:13:07] ------------------------------------------
[01:13:07] ------------------------------------------
[01:13:07] error: Error loading target specification: pre-link-args: expected a JSON object with fields per linker flavor.
[01:13:07]   |
[01:13:07]   = help: Use `--print target-list` for a list of built-in targets
[01:13:07] 
[01:13:07] make[1]: *** [all] Error 101
[01:13:07] ------------------------------------------
[01:13:07] 
[01:13:07] 
[01:13:07] thread '[run-make] run-make-fulldeps/target-specs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[01:13:07] 
[01:13:07] 
[01:13:07] failures:
[01:13:07]     [run-make] run-make-fulldeps/target-specs
[01:13:07]     [run-make] run-make-fulldeps/target-specs
[01:13:07] 
[01:13:07] test result: FAILED. 186 passed; 1 failed; 0 ignored; 0 measurr binaryformat bitreader bitwriter bpf bpfasmprinter bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader lanai lanaiasmparser lanaiasmprinter lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--llvm-cxxflags" "-I/usr/lib/llvm-5.0/include -std=c++0x -fuse-ld=gold -Wl,--no-keep-files-mapped -Wl,--no-map-whole-files -fPIC -fvisibility-inlines-hidden -Werror=date-time -std=c++11 -Wall -W -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wdelete-non-virtual-dtor -Wno-comment -ffunction-sections -fdata-sections -O2 -DNDEBUG -g1  -fno-exceptions -DLLVM_BUILD_GLOBAL_ISEL -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:13:07] 
[01:13:07] 
[01:13:07] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:13:07] Build completed unsuccessfully in 0:31:32
[01:13:07] Build completed unsuccessfully in 0:31:32
[01:13:07] make: *** [check] Error 1
[01:13:07] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:12ce5eb4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
