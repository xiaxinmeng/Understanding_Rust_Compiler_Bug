plain

---- [ui] ui/generic-associated-types/issue-87258_b.rs stdout ----
diff of stderr:

4 LL |     fn foo<'a>() -> Self::FooFuture<'a> {
6    |
6    |
-    = note: hidden type `Struct<'_>` captures lifetime '_#38r
+    = note: hidden type `Struct<'_>` captures lifetime '_#7r
9 error: aborting due to previous error
10 

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-87258_b/issue-87258_b.stderr
To only update this specific test, also pass `--test-args generic-associated-types/issue-87258_b.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/issue-87258_b.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-87258_b" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-87258_b/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
   |
   |
LL |     fn foo<'a>() -> Self::FooFuture<'a> { //~ ERROR
   |
   |
   = note: hidden type `Struct<'_>` captures lifetime '_#7r
error: aborting due to previous error

For more information about this error, try `rustc --explain E0700`.


------------------------------------------


---- [ui] ui/generic-associated-types/issue-87258_a.rs stdout ----
diff of stderr:

4 LL |     fn foo<'a>() -> Self::FooFuture<'a> {
6    |
6    |
-    = note: hidden type `Struct<'_>` captures lifetime '_#38r
+    = note: hidden type `Struct<'_>` captures lifetime '_#7r
9 error: aborting due to previous error
10 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-87258_a/issue-87258_a.stderr
To only update this specific test, also pass `--test-args generic-associated-types/issue-87258_a.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/issue-87258_a.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-87258_a" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-87258_a/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
  --> /checkout/src/test/ui/generic-associated-types/issue-87258_a.rs:19:21
   |
LL |     fn foo<'a>() -> Self::FooFuture<'a> { //~ ERROR
   |
   |
   = note: hidden type `Struct<'_>` captures lifetime '_#7r
error: aborting due to previous error

For more information about this error, try `rustc --explain E0700`.

---

36                                                                 lit
37                                                             } else {
38                                                                 {
-                                                                     ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(&["internal error: entered unreachable code"],
-                                                                                                                                 &match ()
-                                                                                                                                      _args
-                                                                                                                                      =>
-                                                                                                                                      [],
-                                                                                                                                  }))
-                                                                                                                                  }))
+                                                                     ::core::panicking::panic("internal error: entered unreachable code")
47                                                             }
48                                                         })),



The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/quote-debug/quote-debug.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args proc-macro/quote-debug.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/quote-debug.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/quote-debug" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unpretty=expanded" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/quote-debug/auxiliary"
------------------------------------------
#![feature(prelude_import)]
#![no_std]
// check-pass
// check-pass
// force-host
// no-prefer-dynamic
// compile-flags: -Z unpretty=expanded
// This file is not actually used as a proc-macro - instead,
// This file is not actually used as a proc-macro - instead,
// it's just used to show the output of the `quote!` macro

#![feature(proc_macro_quote)]
#![crate_type = "proc-macro"]
#[prelude_import]
use ::std::prelude::rust_2015::*;
extern crate std;

extern crate proc_macro;


fn main() {
    [crate::TokenStream::from(crate::TokenTree::Ident(crate::Ident::new("let",
                                                                        crate::Span::recover_proc_macro_span(0)))),
     crate::TokenStream::from(crate::TokenTree::Ident(crate::Ident::new("hello",
                                                                        crate::Span::recover_proc_macro_span(1)))),
     crate::TokenStream::from(crate::TokenTree::Punct(crate::Punct::new('\u{3d}',
                                                                        crate::Spacing::Alone))),
     crate::TokenStream::from(crate::TokenTree::Literal({
                                                            let mut iter =
                                                                "\"world\"".parse::<crate::TokenStream>().unwrap().into_iter();
                                                            if let (Some(crate::TokenTree::Literal(mut lit)),
                                                                    None) =
                                                                   (iter.next(),
                                                                    iter.next())
                                                               {
                                                                lit.set_span(crate::Span::recover_proc_macro_span(2));
                                                                lit
                                                                {
                                                                {
                                                                    ::core::panicking::panic("internal error: entered unreachable code")
                                                            }
                                                        })),
                                                        })),
     crate::TokenStream::from(crate::TokenTree::Punct(crate::Punct::new('\u{3b}',
                                                                        crate::Spacing::Alone)))].iter().cloned().collect::<crate::TokenStream>()
const _: () =
    {
        extern crate proc_macro;
        #[rustc_proc_macro_decls]
        #[rustc_proc_macro_decls]
        #[allow(deprecated)]
        static _DECLS: &[proc_macro::bridge::client::ProcMacro] = &[];

------------------------------------------
stderr:
------------------------------------------
---
test result: FAILED. 12393 passed; 3 failed; 119 ignored; 0 measured; 0 filtered out; finished in 160.74s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:14:04
