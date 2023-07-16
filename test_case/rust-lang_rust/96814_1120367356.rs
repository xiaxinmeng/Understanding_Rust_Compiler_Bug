plain
---- [ui] src/test/ui/aligned_enum_cast.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/aligned_enum_cast.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/aligned_enum_cast/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/aligned_enum_cast/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: /checkout/compiler/rustc_codegen_ssa/src/mir/operand.rs:116:18: not immediate: OperandRef(Ref((%Aligned*:  %7 = alloca %Aligned, align 8), None, Align(8 bytes)) @ TyAndLayout { ty: Aligned, layout: Layout { fields: Arbitrary { offsets: [Size(0 bytes)], memory_index: [0] }, variants: Multiple { tag: Initialized { value: Int(I8, false), valid_range: 0..=1 }, tag_encoding: Direct, tag_field: 0, variants: [Layout { fields: Arbitrary { offsets: [], memory_index: [] }, variants: Single { index: 0 }, abi: Aggregate { sized: true }, largest_niche: None, align: AbiAndPrefAlign { abi: Align(8 bytes), pref: Align(8 bytes) }, size: Size(8 bytes) }, Layout { fields: Arbitrary { offsets: [], memory_index: [] }, variants: Single { index: 1 }, abi: Aggregate { sized: true }, largest_niche: None, align: AbiAndPrefAlign { abi: Align(8 bytes), pref: Align(8 bytes) }, size: Size(8 bytes) }] }, abi: Aggregate { sized: true }, largest_niche: Some(Niche { offset: Size(0 bytes), value: Int(I8, false), valid_range: 0..=1 }), align: AbiAndPrefAlign { abi: Align(8 bytes), pref: Align(8 bytes) }, size: Size(8 bytes) } })
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1324:9
stack backtrace:
stack backtrace:
   0:     0x7fe1af42a9dc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h613be0f73e798e15
   1:     0x7fe1af490f68 - core::fmt::write::hbbd9ecb2c6a00555
   2:     0x7fe1af41a7a1 - std::io::Write::write_fmt::ha718c2a0404c5b1f
   3:     0x7fe1af42da0e - std::panicking::default_hook::{{closure}}::hf32367d265385281
   4:     0x7fe1af42d63c - std::panicking::default_hook::hde4b4f329a3c0bd6
   5:     0x7fe1affaa771 - rustc_driver[658b754a871b08a7]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fe1af42e26e - std::panicking::rust_panic_with_hook::hf755413905a0f76c
   7:     0x7fe1b2794733 - std[bfd8b0056c42153]::panicking::begin_panic::<rustc_errors[892915e6e9047993]::ExplicitBug>::{closure#0}
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   8:     0x7fe1b2792f36 - std[bfd8b0056c42153]::sys_common::backtrace::__rust_end_short_backtrace::<std[bfd8b0056c42153]::panicking::begin_panic<rustc_errors[892915e6e9047993]::ExplicitBug>::{closure#0}, !>
   9:     0x7fe1afea3ab6 - std[bfd8b0056c42153]::panicking::begin_panic::<rustc_errors[892915e6e9047993]::ExplicitBug>
  10:     0x7fe1b284d8d6 - std[bfd8b0056c42153]::panic::panic_any::<rustc_errors[892915e6e9047993]::ExplicitBug>
  11:     0x7fe1b2848ce6 - <rustc_errors[892915e6e9047993]::HandlerInner>::bug::<&alloc[36b32bef3432d6f4]::string::String>
  12:     0x7fe1b2848570 - <rustc_errors[892915e6e9047993]::Handler>::bug::<&alloc[36b32bef3432d6f4]::string::String>
  13:     0x7fe1b296de35 - rustc_middle[388a32a8dd12158c]::util::bug::opt_span_bug_fmt::<rustc_span[8302cf0de4cf4177]::span_encoding::Span>::{closure#0}
  14:     0x7fe1b296cb0b - rustc_middle[388a32a8dd12158c]::ty::context::tls::with_opt::<rustc_middle[388a32a8dd12158c]::util::bug::opt_span_bug_fmt<rustc_span[8302cf0de4cf4177]::span_encoding::Span>::{closure#0}, ()>::{closure#0}
  15:     0x7fe1b296cabc - rustc_middle[388a32a8dd12158c]::ty::context::tls::with_opt::<rustc_middle[388a32a8dd12158c]::util::bug::opt_span_bug_fmt<rustc_span[8302cf0de4cf4177]::span_encoding::Span>::{closure#0}, ()>
  16:     0x7fe1b296dd79 - rustc_middle[388a32a8dd12158c]::util::bug::opt_span_bug_fmt::<rustc_span[8302cf0de4cf4177]::span_encoding::Span>
  17:     0x7fe1afeaa535 - rustc_middle[388a32a8dd12158c]::util::bug::bug_fmt
  18:     0x7fe1b0293736 - <rustc_codegen_ssa[2580fc8f1fd35c2f]::mir::operand::OperandRef<&rustc_codegen_llvm[678991bf26a40eed]::llvm_::ffi::Value>>::immediate
  19:     0x7fe1b02bbb6b - <rustc_codegen_ssa[2580fc8f1fd35c2f]::mir::FunctionCx<rustc_codegen_llvm[678991bf26a40eed]::builder::Builder>>::codegen_rvalue_operand
  20:     0x7fe1b02b9691 - <rustc_codegen_ssa[2580fc8f1fd35c2f]::mir::FunctionCx<rustc_codegen_llvm[678991bf26a40eed]::builder::Builder>>::codegen_rvalue
  21:     0x7fe1b02c2f37 - <rustc_codegen_ssa[2580fc8f1fd35c2f]::mir::FunctionCx<rustc_codegen_llvm[678991bf26a40eed]::builder::Builder>>::codegen_block
  22:     0x7fe1b02b6626 - rustc_codegen_ssa[2580fc8f1fd35c2f]::mir::codegen_mir::<rustc_codegen_llvm[678991bf26a40eed]::builder::Builder>
  23:     0x7fe1b01ebae6 - rustc_codegen_ssa[2580fc8f1fd35c2f]::base::codegen_instance::<rustc_codegen_llvm[678991bf26a40eed]::builder::Builder>
  24:     0x7fe1b0278ac7 - <rustc_middle[388a32a8dd12158c]::mir::mono::MonoItem as rustc_codegen_ssa[2580fc8f1fd35c2f]::mono_item::MonoItemExt>::define::<rustc_codegen_llvm[678991bf26a40eed]::builder::Builder>
  25:     0x7fe1b01d61b4 - rustc_codegen_llvm[678991bf26a40eed]::base::compile_codegen_unit::module_codegen
  26:     0x7fe1b01d4612 - rustc_codegen_llvm[678991bf26a40eed]::base::compile_codegen_unit
  27:     0x7fe1b01ea456 - rustc_codegen_ssa[2580fc8f1fd35c2f]::base::codegen_crate::<rustc_codegen_llvm[678991bf26a40eed]::LlvmCodegenBackend>
  28:     0x7fe1b02377bb - <rustc_codegen_llvm[678991bf26a40eed]::LlvmCodegenBackend as rustc_codegen_ssa[2580fc8f1fd35c2f]::traits::backend::CodegenBackend>::codegen_crate
  29:     0x7fe1b0093781 - <rustc_session[8fbcc3088d6a2cef]::session::Session>::time::<alloc[36b32bef3432d6f4]::boxed::Box<dyn core[f4e16023ea15bf88]::any::Any>, rustc_interface[468d972b3378653e]::passes::start_codegen::{closure#0}>
  30:     0x7fe1b007bdc3 - <rustc_interface[468d972b3378653e]::passes::QueryContext>::enter::<<rustc_interface[468d972b3378653e]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[f4e16023ea15bf88]::result::Result<alloc[36b32bef3432d6f4]::boxed::Box<dyn core[f4e16023ea15bf88]::any::Any>, rustc_errors[892915e6e9047993]::ErrorGuaranteed>>
  31:     0x7fe1b006748e - <rustc_interface[468d972b3378653e]::queries::Queries>::ongoing_codegen
  32:     0x7fe1aff39ae0 - <rustc_interface[468d972b3378653e]::interface::Compiler>::enter::<rustc_driver[658b754a871b08a7]::run_compiler::{closure#1}::{closure#2}, core[f4e16023ea15bf88]::result::Result<core[f4e16023ea15bf88]::option::Option<rustc_interface[468d972b3378653e]::queries::Linker>, rustc_errors[892915e6e9047993]::ErrorGuaranteed>>
  33:     0x7fe1aff1ae4b - rustc_span[8302cf0de4cf4177]::with_source_map::<core[f4e16023ea15bf88]::result::Result<(), rustc_errors[892915e6e9047993]::ErrorGuaranteed>, rustc_interface[468d972b3378653e]::interface::create_compiler_and_run<core[f4e16023ea15bf88]::result::Result<(), rustc_errors[892915e6e9047993]::ErrorGuaranteed>, rustc_driver[658b754a871b08a7]::run_compiler::{closure#1}>::{closure#1}>
  34:     0x7fe1aff3ac19 - <scoped_tls[169fbf8c86528e79]::ScopedKey<rustc_span[8302cf0de4cf4177]::SessionGlobals>>::set::<rustc_interface[468d972b3378653e]::interface::run_compiler<core[f4e16023ea15bf88]::result::Result<(), rustc_errors[892915e6e9047993]::ErrorGuaranteed>, rustc_driver[658b754a871b08a7]::run_compiler::{closure#1}>::{closure#0}, core[f4e16023ea15bf88]::result::Result<(), rustc_errors[892915e6e9047993]::ErrorGuaranteed>>
  35:     0x7fe1aff96549 - std[bfd8b0056c42153]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[468d972b3378653e]::util::run_in_thread_pool_with_globals<rustc_interface[468d972b3378653e]::interface::run_compiler<core[f4e16023ea15bf88]::result::Result<(), rustc_errors[892915e6e9047993]::ErrorGuaranteed>, rustc_driver[658b754a871b08a7]::run_compiler::{closure#1}>::{closure#0}, core[f4e16023ea15bf88]::result::Result<(), rustc_errors[892915e6e9047993]::ErrorGuaranteed>>::{closure#0}, core[f4e16023ea15bf88]::result::Result<(), rustc_errors[892915e6e9047993]::ErrorGuaranteed>>
  36:     0x7fe1aff3bbe1 - std[bfd8b0056c42153]::panicking::try::<core[f4e16023ea15bf88]::result::Result<(), rustc_errors[892915e6e9047993]::ErrorGuaranteed>, core[f4e16023ea15bf88]::panic::unwind_safe::AssertUnwindSafe<<std[bfd8b0056c42153]::thread::Builder>::spawn_unchecked_<rustc_interface[468d972b3378653e]::util::run_in_thread_pool_with_globals<rustc_interface[468d972b3378653e]::interface::run_compiler<core[f4e16023ea15bf88]::result::Result<(), rustc_errors[892915e6e9047993]::ErrorGuaranteed>, rustc_driver[658b754a871b08a7]::run_compiler::{closure#1}>::{closure#0}, core[f4e16023ea15bf88]::result::Result<(), rustc_errors[892915e6e9047993]::ErrorGuaranteed>>::{closure#0}, core[f4e16023ea15bf88]::result::Result<(), rustc_errors[892915e6e9047993]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  37:     0x7fe1aff914a2 - <<std[bfd8b0056c42153]::thread::Builder>::spawn_unchecked_<rustc_interface[468d972b3378653e]::util::run_in_thread_pool_with_globals<rustc_interface[468d972b3378653e]::interface::run_compiler<core[f4e16023ea15bf88]::result::Result<(), rustc_errors[892915e6e9047993]::ErrorGuaranteed>, rustc_driver[658b754a871b08a7]::run_compiler::{closure#1}>::{closure#0}, core[f4e16023ea15bf88]::result::Result<(), rustc_errors[892915e6e9047993]::ErrorGuaranteed>>::{closure#0}, core[f4e16023ea15bf88]::result::Result<(), rustc_errors[892915e6e9047993]::ErrorGuaranteed>>::{closure#1} as core[f4e16023ea15bf88]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  38:     0x7fe1af4395c3 - std::sys::unix::thread::Thread::new::thread_start::hf79a9752e5110a77
  39:     0x7fe1a998c609 - start_thread
  40:     0x7fe1af29f163 - clone
  41:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.62.0-nightly (7c4fe5f4f 2022-05-08) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
error: aborting due to previous error
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/layout/issue-96185-overaligned-enum.rs stdout ----
diff of stderr:

1 error: layout_of(Aligned1) = Layout {
2            fields: Arbitrary {
3                offsets: [
-                        raw: 0,
-                    },
+                    Size(0 bytes),
7                ],
7                ],
8                memory_index: [
9                    0,

33                        },
34                        largest_niche: None,
35                        align: AbiAndPrefAlign {
-                            abi: Align {
-                                pow2: 3,
-                            },
-                            pref: $PREF_ALIGN,
+                            abi: Align(8 bytes),
+                            pref: Align(8 bytes),
-                        size: Size {
-                            raw: 8,
-                        },
+                        size: Size(8 bytes),
+                        size: Size(8 bytes),
44                    },
45                    Layout {
46                        fields: Arbitrary {

55                        },
56                        largest_niche: None,
57                        align: AbiAndPrefAlign {
-                            abi: Align {
-                                pow2: 3,
-                            },
-                            pref: $PREF_ALIGN,
+                            abi: Align(8 bytes),
+                            pref: Align(8 bytes),
-                        size: Size {
-                            raw: 8,
-                        },
+                        size: Size(8 bytes),
---
73                Niche {
-                    offset: Size {
-                        raw: 0,
-                    },
+                    offset: Size(0 bytes),
77                    value: Int(
79                        false,

82                },
83            ),
83            ),
84            align: AbiAndPrefAlign {
-                abi: Align {
-                    pow2: 3,
-                },
-                pref: $PREF_ALIGN,
+                abi: Align(8 bytes),
+                pref: Align(8 bytes),
-            size: Size {
-                raw: 8,
-            },
+            size: Size(8 bytes),
+            size: Size(8 bytes),
93        }
94   --> $DIR/issue-96185-overaligned-enum.rs:8:1
95    |

102 error: layout_of(Aligned2) = Layout {
103            fields: Arbitrary {
104                offsets: [
-                        raw: 0,
-                    },
+                    Size(0 bytes),
108                ],
108                ],
109                memory_index: [
110                    0,

134                        },
135                        largest_niche: None,
136                        align: AbiAndPrefAlign {
-                            abi: Align {
-                                pow2: 0,
-                            },
-                            pref: $PREF_ALIGN,
+                            abi: Align(1 bytes),
+                            pref: Align(8 bytes),
-                        size: Size {
-                            raw: 1,
-                        },
+                        size: Size(1 bytes),
+                        size: Size(1 bytes),
145                    },
146                    Layout {
147                        fields: Arbitrary {

156                        },
157                        largest_niche: None,
158                        align: AbiAndPrefAlign {
-                            abi: Align {
-                                pow2: 0,
-                            },
-                            pref: $PREF_ALIGN,
+                            abi: Align(1 bytes),
+                            pref: Align(8 bytes),
-                        size: Size {
-                            raw: 1,
-                        },
+                        size: Size(1 bytes),
---
180                Niche {
-                    offset: Size {
-                        raw: 0,
-                    },
+                    offset: Size(0 bytes),
184                    value: Int(
186                        false,

189                },
190            ),
190            ),
191            align: AbiAndPrefAlign {
-                abi: Align {
-                    pow2: 0,
-                },
-                pref: $PREF_ALIGN,
+                abi: Align(1 bytes),
+                pref: Align(8 bytes),
-            size: Size {
-                raw: 1,
-            },
+            size: Size(1 bytes),
---
To only update this specific test, also pass `--test-args layout/issue-96185-overaligned-enum.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/layout/issue-96185-overaligned-enum.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/layout/issue-96185-overaligned-enum" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/layout/issue-96185-overaligned-enum/auxiliary"
stdout: none
--- stderr -------------------------------
error: layout_of(Aligned1) = Layout {
           fields: Arbitrary {
               offsets: [
                   Size(0 bytes),
               memory_index: [
                   0,
               ],
           },
---
                       false,
                   ),
                   valid_range: 0..=1,
               },
               tag_encoding: Direct,
               tag_field: 0,
               variants: [
                       fields: Arbitrary {
                           offsets: [],
                           memory_index: [],
                       },
                       },
                       variants: Single {
                           index: 0,
                       },
                       abi: Aggregate {
                           sized: true,
                       },
                       largest_niche: None,
                       align: AbiAndPrefAlign {
                           abi: Align(8 bytes),
                           pref: Align(8 bytes),
                       size: Size(8 bytes),
                   },
                   Layout {
                       fields: Arbitrary {
---
                       },
                       abi: Aggregate {
                           sized: true,
                       },
                       largest_niche: None,
                       align: AbiAndPrefAlign {
                           abi: Align(8 bytes),
                           pref: Align(8 bytes),
                       size: Size(8 bytes),
                   },
               ],
           },
           },
           abi: Aggregate {
               sized: true,
           },
           largest_niche: Some(
               Niche {
                   offset: Size(0 bytes),
                   value: Int(
                       false,
                   ),
                   valid_range: 0..=1,
               },
               },
           ),
           align: AbiAndPrefAlign {
               abi: Align(8 bytes),
               pref: Align(8 bytes),
           size: Size(8 bytes),
       }
  --> /checkout/src/test/ui/layout/issue-96185-overaligned-enum.rs:8:1
   |
   |
LL | / pub enum Aligned1 { //~ ERROR: layout_of
LL | |     Zero = 0,
LL | |     One = 1,
LL | | }


error: layout_of(Aligned2) = Layout {
           fields: Arbitrary {
               offsets: [
                   Size(0 bytes),
               memory_index: [
                   0,
               ],
           },
---
                       false,
                   ),
                   valid_range: 0..=1,
               },
               tag_encoding: Direct,
               tag_field: 0,
               variants: [
                       fields: Arbitrary {
                           offsets: [],
                           memory_index: [],
                       },
                       },
                       variants: Single {
                           index: 0,
                       },
                       abi: Aggregate {
                           sized: true,
                       },
                       largest_niche: None,
                       align: AbiAndPrefAlign {
                           abi: Align(1 bytes),
                           pref: Align(8 bytes),
                       size: Size(1 bytes),
                   },
                   Layout {
                       fields: Arbitrary {
---
                       },
                       abi: Aggregate {
                           sized: true,
                       },
                       largest_niche: None,
                       align: AbiAndPrefAlign {
                           abi: Align(1 bytes),
                           pref: Align(8 bytes),
                       size: Size(1 bytes),
                   },
               ],
           },
---
                   ),
                   valid_range: 0..=1,
               },
           ),
           largest_niche: Some(
               Niche {
                   offset: Size(0 bytes),
                   value: Int(
                       false,
                   ),
                   valid_range: 0..=1,
               },
               },
           ),
           align: AbiAndPrefAlign {
               abi: Align(1 bytes),
               pref: Align(8 bytes),
           size: Size(1 bytes),
       }
  --> /checkout/src/test/ui/layout/issue-96185-overaligned-enum.rs:16:1
   |
   |
LL | / pub enum Aligned2 { //~ ERROR: layout_of
LL | |     Zero = 0,
LL | |     One = 1,
LL | | }

error: aborting due to 2 previous errors
------------------------------------------

