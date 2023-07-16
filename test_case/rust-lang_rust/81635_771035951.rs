plain
.................................................................................................... 9800/11330
.................................................................................................... 9900/11330
.................................................................................................... 10000/11330
.................................................................................................... 10100/11330
................................................................................FF.F.......F......FF 10200/11330
..F...F...............................i............................................................. 10300/11330
.................................................................................................... 10500/11330
.................................................................................................... 10600/11330
.......................................................................................i............ 10700/11330
.................................................................................................... 10800/11330
---

---- [ui] ui/single-use-lifetime/one-use-in-fn-argument-in-band.rs stdout ----
diff of stderr:

- error: lifetime parameter `'a` only used once
-   --> $DIR/one-use-in-fn-argument-in-band.rs:11:10
+ error: lifetime parameter `'b` only used once
+   --> $DIR/one-use-in-fn-argument-in-band.rs:11:22
3    |
4 LL | fn a(x: &'a u32, y: &'b u32) {
-    |          ^^-
-    |          this lifetime is only used here
-    |          this lifetime is only used here
-    |          help: elide the single-use lifetime
+    |                      ^^-
+    |                      this lifetime is only used here
+    |                      this lifetime is only used here
+    |                      help: elide the single-use lifetime
10 note: the lint level is defined here
11   --> $DIR/one-use-in-fn-argument-in-band.rs:4:9


13 LL | #![deny(single_use_lifetimes)]
15 
15 
- error: lifetime parameter `'b` only used once
-   --> $DIR/one-use-in-fn-argument-in-band.rs:11:22
+ error: lifetime parameter `'a` only used once
+   --> $DIR/one-use-in-fn-argument-in-band.rs:11:10
18    |
19 LL | fn a(x: &'a u32, y: &'b u32) {
-    |                      ^^-
-    |                      this lifetime is only used here
-    |                      this lifetime is only used here
-    |                      help: elide the single-use lifetime
+    |          ^^-
+    |          this lifetime is only used here
+    |          this lifetime is only used here
+    |          help: elide the single-use lifetime
25 error: aborting due to 2 previous errors
26 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/single-use-lifetime/one-use-in-fn-argument-in-band/one-use-in-fn-argument-in-band.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args single-use-lifetime/one-use-in-fn-argument-in-band.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/single-use-lifetime/one-use-in-fn-argument-in-band.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/single-use-lifetime/one-use-in-fn-argument-in-band" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/single-use-lifetime/one-use-in-fn-argument-in-band/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: lifetime parameter `'b` only used once
   |
   |
LL | fn a(x: &'a u32, y: &'b u32) {
   |                      ^^-
   |                      this lifetime is only used here
   |                      this lifetime is only used here
   |                      help: elide the single-use lifetime
note: the lint level is defined here
  --> /checkout/src/test/ui/single-use-lifetime/one-use-in-fn-argument-in-band.rs:4:9
   |
   |
LL | #![deny(single_use_lifetimes)]


error: lifetime parameter `'a` only used once
   |
   |
LL | fn a(x: &'a u32, y: &'b u32) {
   |          ^^-
   |          this lifetime is only used here
   |          this lifetime is only used here
   |          help: elide the single-use lifetime
error: aborting due to 2 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/symbol-names/basic.rs#legacy stdout ----
diff of stderr:

- error: symbol-name(_ZN5basic4main17h4272b3de5e868f5aE)
+ error: symbol-name(_ZN5basic4main17hfcf1daab33c43a6aE)
3    |
3    |
4 LL | #[rustc_symbol_name]
5    | ^^^^^^^^^^^^^^^^^^^^
6 
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
- error: demangling(basic::main::h4272b3de5e868f5a)
+ error: demangling(basic::main::hfcf1daab33c43a6a)
9    |
9    |
10 LL | #[rustc_symbol_name]

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/basic.legacy/basic.legacy.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args symbol-names/basic.rs`

error in revision `legacy`: 1 errors occurred comparing output.
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/symbol-names/basic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "legacy" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/basic.legacy" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "symbol-mangling-version=legacy" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/basic.legacy/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: symbol-name(_ZN5basic4main17hfcf1daab33c43a6aE)
   |
   |
LL | #[rustc_symbol_name]


error: demangling(basic::main::hfcf1daab33c43a6a)
   |
   |
LL | #[rustc_symbol_name]


error: demangling-alt(basic::main)
   |
   |
LL | #[rustc_symbol_name]

error: def-path(main)
  --> /checkout/src/test/ui/symbol-names/basic.rs:15:1
   |
   |
LL | #[rustc_def_path]

error: aborting due to 4 previous errors



------------------------------------------


---- [ui] ui/symbol-names/basic.rs#v0 stdout ----


- error: symbol-name(_RNvCs4fqI2P2rA04_5basic4main)
+ error: symbol-name(_RNvCs21hi0yVfW1J_5basic4main)
3    |
3    |
4 LL | #[rustc_symbol_name]
5    | ^^^^^^^^^^^^^^^^^^^^
6 
6 
- error: demangling(basic[317d481089b8c8fe]::main)
+ error: demangling(basic[17891616a171812d]::main)
9    |
9    |
10 LL | #[rustc_symbol_name]

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/basic.v0/basic.v0.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args symbol-names/basic.rs`

error in revision `v0`: 1 errors occurred comparing output.
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/symbol-names/basic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "v0" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/basic.v0" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "symbol-mangling-version=v0" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/basic.v0/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: symbol-name(_RNvCs21hi0yVfW1J_5basic4main)
   |
   |
LL | #[rustc_symbol_name]


error: demangling(basic[17891616a171812d]::main)
   |
   |
LL | #[rustc_symbol_name]


error: demangling-alt(basic::main)
   |
   |
LL | #[rustc_symbol_name]

error: def-path(main)
  --> /checkout/src/test/ui/symbol-names/basic.rs:15:1
   |
   |
LL | #[rustc_def_path]

error: aborting due to 4 previous errors



------------------------------------------


---- [ui] ui/symbol-names/const-generics-demangling.rs stdout ----
diff of stderr:

- error: symbol-name(_RMCs4fqI2P2rA04_25const_generics_demanglingINtB0_8UnsignedKhb_E)
+ error: symbol-name(_RMCs21hi0yVfW1J_25const_generics_demanglingINtB0_8UnsignedKhb_E)
3    |
3    |
4 LL | #[rustc_symbol_name]
5    | ^^^^^^^^^^^^^^^^^^^^
6 
6 
- error: demangling(<const_generics_demangling[317d481089b8c8fe]::Unsigned<11: u8>>)
+ error: demangling(<const_generics_demangling[17891616a171812d]::Unsigned<11: u8>>)
9    |
9    |
10 LL | #[rustc_symbol_name]

16 LL | #[rustc_symbol_name]
18 
18 
- error: symbol-name(_RMs_Cs4fqI2P2rA04_25const_generics_demanglingINtB2_6SignedKsn98_E)
+ error: symbol-name(_RMs_Cs21hi0yVfW1J_25const_generics_demanglingINtB2_6SignedKsn98_E)
21    |
21    |
22 LL | #[rustc_symbol_name]
23    | ^^^^^^^^^^^^^^^^^^^^
24 
24 
- error: demangling(<const_generics_demangling[317d481089b8c8fe]::Signed<-152: i16>>)
+ error: demangling(<const_generics_demangling[17891616a171812d]::Signed<-152: i16>>)
27    |
27    |
28 LL | #[rustc_symbol_name]

34 LL | #[rustc_symbol_name]
36 
36 
- error: symbol-name(_RMs0_Cs4fqI2P2rA04_25const_generics_demanglingINtB3_4BoolKb1_E)
+ error: symbol-name(_RMs0_Cs21hi0yVfW1J_25const_generics_demanglingINtB3_4BoolKb1_E)
39    |
39    |
40 LL | #[rustc_symbol_name]
41    | ^^^^^^^^^^^^^^^^^^^^
42 
42 
- error: demangling(<const_generics_demangling[317d481089b8c8fe]::Bool<true: bool>>)
+ error: demangling(<const_generics_demangling[17891616a171812d]::Bool<true: bool>>)
45    |
45    |
46 LL | #[rustc_symbol_name]

52 LL | #[rustc_symbol_name]
54 
54 
- error: symbol-name(_RMs1_Cs4fqI2P2rA04_25const_generics_demanglingINtB3_4CharKc2202_E)
+ error: symbol-name(_RMs1_Cs21hi0yVfW1J_25const_generics_demanglingINtB3_4CharKc2202_E)
57    |
57    |
58 LL | #[rustc_symbol_name]
59    | ^^^^^^^^^^^^^^^^^^^^
60 
60 
- error: demangling(<const_generics_demangling[317d481089b8c8fe]::Char<'∂': char>>)
+ error: demangling(<const_generics_demangling[17891616a171812d]::Char<'∂': char>>)
63    |
63    |
64 LL | #[rustc_symbol_name]

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/const-generics-demangling/const-generics-demangling.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/const-generics-demangling/const-generics-demangling.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args symbol-names/const-generics-demangling.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/symbol-names/const-generics-demangling.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/const-generics-demangling" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "symbol-mangling-version=v0" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/const-generics-demangling/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: symbol-name(_RMCs21hi0yVfW1J_25const_generics_demanglingINtB0_8UnsignedKhb_E)
   |
   |
LL | #[rustc_symbol_name]


error: demangling(<const_generics_demangling[17891616a171812d]::Unsigned<11: u8>>)
   |
   |
LL | #[rustc_symbol_name]


error: demangling-alt(<const_generics_demangling::Unsigned<11>>)
   |
   |
LL | #[rustc_symbol_name]


error: symbol-name(_RMs_Cs21hi0yVfW1J_25const_generics_demanglingINtB2_6SignedKsn98_E)
   |
   |
LL | #[rustc_symbol_name]


error: demangling(<const_generics_demangling[17891616a171812d]::Signed<-152: i16>>)
   |
   |
LL | #[rustc_symbol_name]


error: demangling-alt(<const_generics_demangling::Signed<-152>>)
   |
   |
LL | #[rustc_symbol_name]


error: symbol-name(_RMs0_Cs21hi0yVfW1J_25const_generics_demanglingINtB3_4BoolKb1_E)
   |
   |
LL | #[rustc_symbol_name]


error: demangling(<const_generics_demangling[17891616a171812d]::Bool<true: bool>>)
   |
   |
LL | #[rustc_symbol_name]


error: demangling-alt(<const_generics_demangling::Bool<true>>)
   |
   |
LL | #[rustc_symbol_name]


error: symbol-name(_RMs1_Cs21hi0yVfW1J_25const_generics_demanglingINtB3_4CharKc2202_E)
   |
   |
LL | #[rustc_symbol_name]


error: demangling(<const_generics_demangling[17891616a171812d]::Char<'∂': char>>)
   |
   |
LL | #[rustc_symbol_name]


error: demangling-alt(<const_generics_demangling::Char<'∂'>>)
   |
   |
LL | #[rustc_symbol_name]

error: aborting due to 12 previous errors



------------------------------------------


---- [ui] ui/symbol-names/impl1.rs#legacy stdout ----
diff of stderr:

- error: symbol-name(_ZN5impl13foo3Foo3bar17ha318160f105e638cE)
+ error: symbol-name(_ZN5impl13foo3Foo3bar17h7dfbb7a147ef1ac6E)
3    |
3    |
4 LL |         #[rustc_symbol_name]
5    |         ^^^^^^^^^^^^^^^^^^^^
6 
6 
- error: demangling(impl1::foo::Foo::bar::ha318160f105e638c)
+ error: demangling(impl1::foo::Foo::bar::h7dfbb7a147ef1ac6)
9    |
9    |
10 LL |         #[rustc_symbol_name]

22 LL |         #[rustc_def_path]
24 
24 
- error: symbol-name(_ZN5impl13bar33_$LT$impl$u20$impl1..foo..Foo$GT$3baz17h6c2dbab6e66f9fa3E)
+ error: symbol-name(_ZN5impl13bar33_$LT$impl$u20$impl1..foo..Foo$GT$3baz17h797b8f7cb42723b8E)
27    |
27    |
28 LL |         #[rustc_symbol_name]
29    |         ^^^^^^^^^^^^^^^^^^^^
30 
30 
- error: demangling(impl1::bar::<impl impl1::foo::Foo>::baz::h6c2dbab6e66f9fa3)
+ error: demangling(impl1::bar::<impl impl1::foo::Foo>::baz::h797b8f7cb42723b8)
33    |
33    |
34 LL |         #[rustc_symbol_name]

46 LL |         #[rustc_def_path]
48 
48 
- error: symbol-name(_ZN209_$LT$$u5b$$RF$dyn$u20$impl1..Foo$u2b$Assoc$u20$$u3d$$u20$extern$u20$$u22$C$u22$$u20$fn$LP$$RF$u8$C$$u20$...$RP$$u2b$impl1..AutoTrait$u3b$$u20$3$u5d$$u20$as$u20$impl1..main..$u7b$$u7b$closure$u7d$$u7d$..Bar$GT$6method17SYMBOL_HASHE)
+ error: symbol-name(_ZN209_$LT$$u5b$$RF$dyn$u20$impl1..Foo$u2b$Assoc$u20$$u3d$$u20$extern$u20$$u22$C$u22$$u20$fn$LP$$RF$u8$C$$u20$...$RP$$u2b$impl1..AutoTrait$u3b$$u20$3$u5d$$u20$as$u20$impl1..main..$u7b$$u7b$closure$u7d$$u7d$..Bar$GT$6method17h7abf9fe633758d74E)
51    |
51    |
52 LL |             #[rustc_symbol_name]
53    |             ^^^^^^^^^^^^^^^^^^^^
54 
54 
- error: demangling(<[&dyn impl1::Foo+Assoc = extern "C" fn(&u8, ::.)+impl1::AutoTrait; 3] as impl1::main::{{closure}}::Bar>::method::SYMBOL_HASH)
+ error: demangling(<[&dyn impl1::Foo+Assoc = extern "C" fn(&u8, ::.)+impl1::AutoTrait; 3] as impl1::main::{{closure}}::Bar>::method::h7abf9fe633758d74)
57    |
57    |
58 LL |             #[rustc_symbol_name]

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/impl1.legacy/impl1.legacy.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args symbol-names/impl1.rs`

error in revision `legacy`: 1 errors occurred comparing output.
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/symbol-names/impl1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "legacy" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/impl1.legacy" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "symbol-mangling-version=legacy" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/impl1.legacy/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: symbol-name(_ZN5impl13foo3Foo3bar17h7dfbb7a147ef1ac6E)
   |
   |
LL |         #[rustc_symbol_name]


error: demangling(impl1::foo::Foo::bar::h7dfbb7a147ef1ac6)
   |
   |
LL |         #[rustc_symbol_name]


error: demangling-alt(impl1::foo::Foo::bar)
   |
   |
LL |         #[rustc_symbol_name]


error: def-path(foo::Foo::bar)
   |
   |
LL |         #[rustc_def_path]


error: symbol-name(_ZN5impl13bar33_$LT$impl$u20$impl1..foo..Foo$GT$3baz17h797b8f7cb42723b8E)
   |
   |
LL |         #[rustc_symbol_name]


error: demangling(impl1::bar::<impl impl1::foo::Foo>::baz::h797b8f7cb42723b8)
   |
   |
LL |         #[rustc_symbol_name]


error: demangling-alt(impl1::bar::<impl impl1::foo::Foo>::baz)
   |
   |
LL |         #[rustc_symbol_name]


error: def-path(bar::<impl foo::Foo>::baz)
   |
   |
LL |         #[rustc_def_path]


error: symbol-name(_ZN209_$LT$$u5b$$RF$dyn$u20$impl1..Foo$u2b$Assoc$u20$$u3d$$u20$extern$u20$$u22$C$u22$$u20$fn$LP$$RF$u8$C$$u20$...$RP$$u2b$impl1..AutoTrait$u3b$$u20$3$u5d$$u20$as$u20$impl1..main..$u7b$$u7b$closure$u7d$$u7d$..Bar$GT$6method17h7abf9fe633758d74E)
   |
   |
LL |             #[rustc_symbol_name]


error: demangling(<[&dyn impl1::Foo+Assoc = extern "C" fn(&u8, ::.)+impl1::AutoTrait; 3] as impl1::main::{{closure}}::Bar>::method::h7abf9fe633758d74)
   |
   |
LL |             #[rustc_symbol_name]


---
test result: FAILED. 11233 passed; 9 failed; 88 ignored; 0 measured; 0 filtered out; finished in 112.86s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:12:28
