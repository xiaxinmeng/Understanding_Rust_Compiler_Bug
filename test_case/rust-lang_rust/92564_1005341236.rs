plain
failures:

---- [rustdoc] rustdoc/intra-doc/prim-associated-traits.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/prim-associated-traits" "/checkout/src/test/rustdoc/intra-doc/prim-associated-traits.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
4: @has check failed
 `XPATH PATTERN` did not match
 // @has 'prim_associated_traits/struct.Number.html' '//a[@href="https://doc.rust-lang.org/nightly/std/primitive.f64.html#method.from_str"]' 'f64::from_str()'
5: @has check failed
 `XPATH PATTERN` did not match
 // @has 'prim_associated_traits/struct.Number.html' '//a[@href="https://doc.rust-lang.org/nightly/std/primitive.f32.html#method.from_str"]' 'f32::from_str()'
6: @has check failed
 `XPATH PATTERN` did not match
 // @has 'prim_associated_traits/struct.Number.html' '//a[@href="https://doc.rust-lang.org/nightly/std/primitive.isize.html#method.from_str"]' 'isize::from_str()'
7: @has check failed
 `XPATH PATTERN` did not match
 // @has 'prim_associated_traits/struct.Number.html' '//a[@href="https://doc.rust-lang.org/nightly/std/primitive.i8.html#method.from_str"]' 'i8::from_str()'
8: @has check failed
 `XPATH PATTERN` did not match
 // @has 'prim_associated_traits/struct.Number.html' '//a[@href="https://doc.rust-lang.org/nightly/std/primitive.i16.html#method.from_str"]' 'i16::from_str()'
9: @has check failed
 `XPATH PATTERN` did not match
 // @has 'prim_associated_traits/struct.Number.html' '//a[@href="https://doc.rust-lang.org/nightly/std/primitive.i32.html#method.from_str"]' 'i32::from_str()'
10: @has check failed
 `XPATH PATTERN` did not match
 // @has 'prim_associated_traits/struct.Number.html' '//a[@href="https://doc.rust-lang.org/nightly/std/primitive.i64.html#method.from_str"]' 'i64::from_str()'
11: @has check failed
 `XPATH PATTERN` did not match
 // @has 'prim_associated_traits/struct.Number.html' '//a[@href="https://doc.rust-lang.org/nightly/std/primitive.i128.html#method.from_str"]' 'i128::from_str()'
12: @has check failed
 `XPATH PATTERN` did not match
 // @has 'prim_associated_traits/struct.Number.html' '//a[@href="https://doc.rust-lang.org/nightly/std/primitive.usize.html#method.from_str"]' 'usize::from_str()'
13: @has check failed
 `XPATH PATTERN` did not match
 // @has 'prim_associated_traits/struct.Number.html' '//a[@href="https://doc.rust-lang.org/nightly/std/primitive.u8.html#method.from_str"]' 'u8::from_str()'
14: @has check failed
 `XPATH PATTERN` did not match
 // @has 'prim_associated_traits/struct.Number.html' '//a[@href="https://doc.rust-lang.org/nightly/std/primitive.u16.html#method.from_str"]' 'u16::from_str()'
15: @has check failed
 `XPATH PATTERN` did not match
 // @has 'prim_associated_traits/struct.Number.html' '//a[@href="https://doc.rust-lang.org/nightly/std/primitive.u32.html#method.from_str"]' 'u32::from_str()'
16: @has check failed
 `XPATH PATTERN` did not match
 // @has 'prim_associated_traits/struct.Number.html' '//a[@href="https://doc.rust-lang.org/nightly/std/primitive.u64.html#method.from_str"]' 'u64::from_str()'
17: @has check failed
 `XPATH PATTERN` did not match
 // @has 'prim_associated_traits/struct.Number.html' '//a[@href="https://doc.rust-lang.org/nightly/std/primitive.u128.html#method.from_str"]' 'u128::from_str()'
18: @has check failed
 `XPATH PATTERN` did not match
 // @has 'prim_associated_traits/struct.Number.html' '//a[@href="https://doc.rust-lang.org/nightly/std/primitive.char.html#method.from_str"]' 'char::from_str()'
19: @has check failed
 `XPATH PATTERN` did not match
 // @has 'prim_associated_traits/struct.Number.html' '//a[@href="https://doc.rust-lang.org/nightly/std/primitive.bool.html#method.from_str"]' 'bool::from_str()'
20: @has check failed
 `XPATH PATTERN` did not match
 // @has 'prim_associated_traits/struct.Number.html' '//a[@href="https://doc.rust-lang.org/nightly/std/primitive.str.html#method.eq"]' 'str::eq()'
21: @has check failed
 `XPATH PATTERN` did not match
 // @has 'prim_associated_traits/struct.Number.html' '//a[@href="https://doc.rust-lang.org/nightly/std/primitive.never.html#method.eq"]' 'never::eq()'
Encountered 18 errors

------------------------------------------

---
test result: FAILED. 488 passed; 1 failed; 2 ignored; 0 measured; 0 filtered out; finished in 31.95s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "rustdoc" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "13.0.0-rust-1.59.0-stable" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwp engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto m68k m68kasmparser m68kcodegen m68kdesc m68kdisassembler m68kinfo mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "stable" "--color" "always"


Build completed unsuccessfully in 0:21:12
