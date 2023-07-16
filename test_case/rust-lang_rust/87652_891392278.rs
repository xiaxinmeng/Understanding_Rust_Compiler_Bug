plain

296    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
297    = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>
298 
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
- warning: inlining naked functions is unsupported
+ warning: naked functions cannot be inlined
300   --> $DIR/naked-functions.rs:177:1
302 LL | #[inline]

305    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
306    = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>
306    = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>
307 
- warning: inlining naked functions is unsupported
+ warning: naked functions cannot be inlined
309   --> $DIR/naked-functions.rs:185:1
310    |
311 LL | #[inline(always)]
314    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
315    = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>
316 
- warning: inlining naked functions is unsupported
- warning: inlining naked functions is unsupported
+ warning: naked functions cannot be inlined
318   --> $DIR/naked-functions.rs:193:1
319    |
320 LL | #[inline(never)]
323    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
324    = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>
325 
- warning: inlining naked functions is unsupported
- warning: inlining naked functions is unsupported
+ warning: naked functions cannot be inlined
327   --> $DIR/naked-functions.rs:201:1
329 LL | #[inline]

332    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
333    = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>
333    = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>
334 
- warning: inlining naked functions is unsupported
+ warning: naked functions cannot be inlined
336   --> $DIR/naked-functions.rs:204:1
337    |
338 LL | #[inline(always)]
341    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
342    = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>
343 
- warning: inlining naked functions is unsupported
- warning: inlining naked functions is unsupported
+ warning: naked functions cannot be inlined
345   --> $DIR/naked-functions.rs:207:1
346    |
347 LL | #[inline(never)]

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/naked-functions/naked-functions.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args asm/naked-functions.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/naked-functions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/naked-functions" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/naked-functions/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: asm with `pure` option must have at least one output
   |
   |
LL |     asm!("", options(readonly, nostack), options(pure));

error: patterns not allowed in naked function parameters
  --> /checkout/src/test/ui/asm/naked-functions.rs:13:5
   |
   |
LL |     mut a: u32,
   |     ^^^^^

error: patterns not allowed in naked function parameters
  --> /checkout/src/test/ui/asm/naked-functions.rs:15:5
   |
LL |     &b: &i32,

error: patterns not allowed in naked function parameters
  --> /checkout/src/test/ui/asm/naked-functions.rs:17:6
   |
   |
LL |     (None | Some(_)): Option<std::ptr::NonNull<u8>>,

error: patterns not allowed in naked function parameters
  --> /checkout/src/test/ui/asm/naked-functions.rs:19:5
   |
   |
LL |     P { x, y }: P,

error: referencing function parameters is not allowed in naked functions
  --> /checkout/src/test/ui/asm/naked-functions.rs:29:5
   |
   |
LL |     a + 1
   |     ^
   |
   = help: follow the calling convention in asm block to use parameters

warning: naked functions must contain a single asm block
   |
   |
LL | / pub unsafe extern "C" fn inc(a: u32) -> u32 {
LL | |     //~^ WARN naked functions must contain a single asm block
LL | |     //~| WARN this was previously accepted
LL | |     a + 1
   | |     ----- non-asm is unsupported in naked functions
LL | |     //~^ ERROR referencing function parameters is not allowed in naked functions
LL | | }
   |
   = note: `#[warn(unsupported_naked_functions)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>

error: referencing function parameters is not allowed in naked functions
  --> /checkout/src/test/ui/asm/naked-functions.rs:35:31
   |
LL |     asm!("/* {0} */", in(reg) a, options(noreturn));
   |
   |
   = help: follow the calling convention in asm block to use parameters

warning: only `const` and `sym` operands are supported in naked functions
   |
   |
LL |     asm!("/* {0} */", in(reg) a, options(noreturn));
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>


warning: naked functions must contain a single asm block
   |
   |
LL | / pub unsafe extern "C" fn inc_closure(a: u32) -> u32 {
LL | |     //~^ WARN naked functions must contain a single asm block
LL | |     //~| WARN this was previously accepted
LL | |     (|| a + 1)()
   | |     ------------ non-asm is unsupported in naked functions
LL | | }
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>


warning: only `const` and `sym` operands are supported in naked functions
   |
   |
LL |          in(reg) a,
...
...
LL |          inlateout(reg) b,
   |          ^^^^^^^^^^^^^^^^
LL |          inout(reg) c,
   |          ^^^^^^^^^^^^
LL |          lateout(reg) d,
   |          ^^^^^^^^^^^^^^
LL |          out(reg) e,
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>


warning: asm in naked functions must use `noreturn` option
   |
   |
LL | /     asm!("/* {0} {1} {2} {3} {4} {5} {6} */",
LL | |          //~^ WARN asm in naked functions must use `noreturn` option
LL | |          //~| WARN this was previously accepted
LL | |          in(reg) a,
LL | |          sym G,
LL | |     );
   | |______^
   |
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>

warning: naked functions must contain a single asm block
   |
   |
LL | / pub unsafe extern "C" fn unsupported_operands() {
LL | |     //~^ WARN naked functions must contain a single asm block
LL | |     //~| WARN this was previously accepted
LL | |     let mut a = 0usize;
   | |     ------------------- non-asm is unsupported in naked functions
LL | |     let mut b = 0usize;
   | |     ------------------- non-asm is unsupported in naked functions
LL | |     let mut c = 0usize;
   | |     ------------------- non-asm is unsupported in naked functions
LL | |     let mut d = 0usize;
   | |     ------------------- non-asm is unsupported in naked functions
LL | |     let mut e = 0usize;
   | |     ------------------- non-asm is unsupported in naked functions
LL | |     );
LL | | }
   | |_^
   |
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>

warning: naked functions must contain a single asm block
   |
   |
LL | / pub extern "C" fn missing_assembly() {
LL | |     //~^ WARN naked functions must contain a single asm block
LL | |     //~| WARN this was previously accepted
LL | | }
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>


warning: asm in naked functions must use `noreturn` option
   |
   |
LL |     asm!("");
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>


warning: asm in naked functions must use `noreturn` option
   |
   |
LL |     asm!("");
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>


warning: asm in naked functions must use `noreturn` option
   |
   |
LL |     asm!("");
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>


warning: naked functions must contain a single asm block
   |
   |
LL | / pub extern "C" fn too_many_asm_blocks() {
LL | |     //~^ WARN naked functions must contain a single asm block
LL | |     //~| WARN this was previously accepted
LL | |     asm!("");
...  |
LL | |     asm!("");
   | |     --------- multiple asm blocks are unsupported in naked functions
...  |
LL | |     asm!("");
   | |     --------- multiple asm blocks are unsupported in naked functions
...  |
LL | |     asm!("", options(noreturn));
   | |     ---------------------------- multiple asm blocks are unsupported in naked functions
LL | | }
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>


error: referencing function parameters is not allowed in naked functions
  --> /checkout/src/test/ui/asm/naked-functions.rs:101:11
   |
LL |         *&y
   |           ^
   |
   = help: follow the calling convention in asm block to use parameters

warning: naked functions must contain a single asm block
   |
   |
LL | /     pub extern "C" fn inner(y: usize) -> usize {
LL | |         //~^ WARN naked functions must contain a single asm block
LL | |         //~| WARN this was previously accepted
LL | |         *&y
   | |         --- non-asm is unsupported in naked functions
LL | |         //~^ ERROR referencing function parameters is not allowed in naked functions
LL | |     }
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>


warning: the LLVM-style inline assembly is unsupported in naked functions
   |
   |
LL |     llvm_asm!("");
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>
   = help: use the new asm! syntax specified in RFC 2873
   = note: this warning originates in the macro `llvm_asm` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: naked functions must contain a single asm block
   |
   |
LL | / unsafe extern "C" fn llvm() -> ! {
LL | |     //~^ WARN naked functions must contain a single asm block
LL | |     //~| WARN this was previously accepted
LL | |     llvm_asm!("");
LL | |     core::hint::unreachable_unchecked();
LL | |     core::hint::unreachable_unchecked();
   | |     ------------------------------------ non-asm is unsupported in naked functions
LL | | }
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>


warning: asm options unsupported in naked functions: `nomem`, `preserves_flags`
   |
   |
LL |     asm!("", options(nomem, preserves_flags, noreturn));
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>


warning: asm options unsupported in naked functions: `nostack`, `pure`, `readonly`
   |
   |
LL |     asm!("", options(readonly, nostack), options(pure));
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>


warning: asm in naked functions must use `noreturn` option
   |
   |
LL |     asm!("", options(readonly, nostack), options(pure));
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>


warning: Rust ABI is unsupported in naked functions
  --> /checkout/src/test/ui/asm/naked-functions.rs:135:15
   |
LL | pub unsafe fn default_abi() {
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>


warning: Rust ABI is unsupported in naked functions
  --> /checkout/src/test/ui/asm/naked-functions.rs:142:29
   |
LL | pub unsafe extern "Rust" fn rust_abi() {
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>

---

warning: naked functions cannot be inlined
  --> /checkout/src/test/ui/asm/naked-functions.rs:185:1
   |
LL | #[inline(always)]
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>


warning: naked functions cannot be inlined
  --> /checkout/src/test/ui/asm/naked-functions.rs:193:1
   |
LL | #[inline(never)]
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>

---

warning: naked functions cannot be inlined
  --> /checkout/src/test/ui/asm/naked-functions.rs:204:1
   |
LL | #[inline(always)]
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>


warning: naked functions cannot be inlined
  --> /checkout/src/test/ui/asm/naked-functions.rs:207:1
   |
LL | #[inline(never)]
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>

---
test result: FAILED. 11993 passed; 1 failed; 103 ignored; 0 measured; 0 filtered out; finished in 123.74s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:12:28
