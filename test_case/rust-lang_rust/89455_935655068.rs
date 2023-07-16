plain
Successfully built 67bd90dad13f
Successfully tagged rust-ci:latest
Built container sha256:67bd90dad13fd11523440e67aa27f82d1e7310ecec536f833f62099c744e236e
Uploading finished image to https://ci-caches.rust-lang.org/docker/dfd7203a0b015711c96f25420d9cb51dd6d72a416dd27c32932eb6b4d7efea10392bba63f0eaa514ea019391488096f30c8a7ead06c758f8f033ddf38b7029a7
upload failed: - to s3://rust-lang-ci-sccache2/docker/dfd7203a0b015711c96f25420d9cb51dd6d72a416dd27c32932eb6b4d7efea10392bba63f0eaa514ea019391488096f30c8a7ead06c758f8f033ddf38b7029a7 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-10]
---
i................................................................................................... 8400/12254
.......................................i............................................................ 8500/12254
.................................................................................................... 8600/12254
........................i........................................................................... 8700/12254
......................F.....................................................F.F..................... 8800/12254
.................................................................................................... 9000/12254
.................................................................................................... 9100/12254
..............................iiii.iiiii............................................................ 9200/12254
.....ii...............i............................................................................. 9300/12254
---
---- [ui] ui/closures/2229_closure_analysis/filter-on-struct-member.rs stdout ----

error: warning: diagnostic messages should start with lowercase letters
  |
1 | First Pass analysis includes:
  | - this is an uppercase letter
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closures/2229_closure_analysis/filter-on-struct-member.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/filter-on-struct-member" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2021" "-Zunstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/filter-on-struct-member/auxiliary"
------------------------------------------

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
------------------------------------------
------------------------------------------
stderr:
------------------------------------------
error: First Pass analysis includes:
   |
   |
LL |             |v| self.filter.allowed(*v),
   |
   |
note: Capturing self[Deref,(0, 0)] -> ImmBorrow
   |
   |
LL |             |v| self.filter.allowed(*v),


error: Min Capture analysis includes:
   |
   |
LL |             |v| self.filter.allowed(*v),
   |
   |
note: Min Capture self[Deref,(0, 0)] -> ImmBorrow
   |
   |
LL |             |v| self.filter.allowed(*v),

error: aborting due to 2 previous errors



------------------------------------------


---- [ui] ui/linkage-attr/issue-10755.rs stdout ----

error: warning: diagnostic messages should start with lowercase letters
  |
1 | No such file or directory (os error 2)
  | - this is an uppercase letter
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/linkage-attr/issue-10755.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/linkage-attr/issue-10755" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "linker=llllll" "-C" "linker-flavor=ld" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/linkage-attr/issue-10755/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: linker `llllll` not found
   |
   = note: No such file or directory (os error 2)
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/lint/issue-86600-lint-twice.rs stdout ----

error: warning: the word `illegal` is illegal
  |
1 | `#[warn(illegal_floating_point_literal_pattern)]` on by default
  |         ------- consider using more specific word, like `invalid`
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/issue-86600-lint-twice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/issue-86600-lint-twice" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/issue-86600-lint-twice/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: floating-point types cannot be used in patterns
  --> /checkout/src/test/ui/lint/issue-86600-lint-twice.rs:10:9
   |
LL |         5.0 => {}
   |         ^^^
   |
   = note: `#[warn(illegal_floating_point_literal_pattern)]` on by default
   = note: for more information, see issue #41620 <https://github.com/rust-lang/rust/issues/41620>

warning: 1 warning emitted

---
---- [ui] ui/lto-duplicate-symbols.rs stdout ----

error: warning: diagnostic messages should start with lowercase letters
  |
1 | Linking globals named 'foo': symbol multiply defined!
  | - this is an uppercase letter
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lto-duplicate-symbols.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lto-duplicate-symbols" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "lto" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lto-duplicate-symbols/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: Linking globals named 'foo': symbol multiply defined!

error: failed to load bc of "lto-duplicate-symbols2.lto_duplicate_symbols2.693a75b4-cgu.0.rcgu.o": 
error: aborting due to previous error; 1 warning emitted


------------------------------------------
------------------------------------------


---- [ui] ui/macros/local-ambiguity-multiple-parsing-options.rs stdout ----

error: warning: diagnostic messages should not end with punctuations
  |
1 | local ambiguity when calling macro `ambiguity`: multiple parsing options: built-in NTs ident ('i') or ident ('j').
  |                                                                                                                  - this is a punctuation
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/local-ambiguity-multiple-parsing-options.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/local-ambiguity-multiple-parsing-options" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/local-ambiguity-multiple-parsing-options/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: local ambiguity when calling macro `ambiguity`: multiple parsing options: built-in NTs ident ('i') or ident ('j').
   |
   |
LL | ambiguity!(error); //~ ERROR local ambiguity


error: local ambiguity when calling macro `ambiguity`: multiple parsing options: built-in NTs ident ('i') or ident ('j').
   |
   |
LL | ambiguity!(error); //~ ERROR local ambiguity

error: aborting due to 2 previous errors



------------------------------------------


---- [ui] ui/parser/recover-from-homoglyph.rs stdout ----

error: warning: diagnostic messages should start with lowercase letters
  |
1 | Unicode character ';' (Greek Question Mark) looks like ';' (Semicolon), but it is not
  | - this is an uppercase letter
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/recover-from-homoglyph.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/recover-from-homoglyph" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/recover-from-homoglyph/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: unknown start of token: \u{37e}
   |
   |
LL |     println!(""); //~ ERROR unknown start of token: \u{37e}
   |
   |
help: Unicode character ';' (Greek Question Mark) looks like ';' (Semicolon), but it is not
   |
LL |     println!(""); //~ ERROR unknown start of token: \u{37e}

error[E0308]: mismatched types
  --> /checkout/src/test/ui/parser/recover-from-homoglyph.rs:3:20
   |
   |
LL |     let x: usize = (); //~ ERROR mismatched types
   |            -----   ^^ expected `usize`, found `()`
   |            expected due to this

error: aborting due to 2 previous errors

---
---- [ui] ui/parser/unicode-quote-chars.rs stdout ----

error: warning: diagnostic messages should start with lowercase letters
  |
1 | Unicode characters '“' (Left Double Quotation Mark) and '”' (Right Double Quotation Mark) look like '"' (Quotation Mark), but are not
  | - this is an uppercase letter
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/unicode-quote-chars.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/unicode-quote-chars" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/unicode-quote-chars/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: unknown start of token: \u{201c}
   |
   |
LL |     println!(“hello world”);
   |
   |
help: Unicode characters '“' (Left Double Quotation Mark) and '”' (Right Double Quotation Mark) look like '"' (Quotation Mark), but are not
LL |     println!("hello world");
   |              ~~~~~~~~~~~~~


error: unknown start of token: \u{201d}
   |
   |
LL |     println!(“hello world”);
   |
   |
help: Unicode character '”' (Right Double Quotation Mark) looks like '"' (Quotation Mark), but it is not
   |
LL |     println!(“hello world");


error: expected `,`, found `world`
   |
   |
LL |     println!(“hello world”);
   |                     ^^^^^ expected `,`
error: aborting due to 3 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/parser/unicode-chars.rs stdout ----

error: warning: diagnostic messages should start with lowercase letters
  |
1 | Unicode character ';' (Greek Question Mark) looks like ';' (Semicolon), but it is not
  | - this is an uppercase letter
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/unicode-chars.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/unicode-chars" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/unicode-chars/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: unknown start of token: \u{37e}
   |
LL |     let y = 0;
   |              ^
   |
   |
help: Unicode character ';' (Greek Question Mark) looks like ';' (Semicolon), but it is not
LL |     let y = 0;
   |              ~

error: aborting due to previous error
---
---- [ui] ui/span/issue-81800.rs stdout ----

error: warning: diagnostic messages should start with lowercase letters
  |
1 | Unicode character '˂' (Modifier Letter Left Arrowhead) looks like '<' (Less-Than Sign), but it is not
  | - this is an uppercase letter
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/issue-81800.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-81800" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-81800/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: unknown start of token: \u{2c2}
   |
   |
LL | fn x˂- //~ ERROR: unknown start of token
   |
   |
help: Unicode character '˂' (Modifier Letter Left Arrowhead) looks like '<' (Less-Than Sign), but it is not
   |
LL | fn x<- //~ ERROR: unknown start of token


error: expected one of `#`, `>`, `const`, identifier, or lifetime, found `-`
   |
   |
LL | fn x˂- //~ ERROR: unknown start of token
   |      ^ expected one of `#`, `>`, `const`, identifier, or lifetime
error: aborting due to 2 previous errors


------------------------------------------
---
test result: FAILED. 12130 passed; 9 failed; 115 ignored; 0 measured; 0 filtered out; finished in 134.40s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:27
