plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:2efbfac82b677cf89dacd548a824f7377c8d4e77)
Complete job name: PR (x86_64-gnu-llvm-13, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-13
---
---- [rustdoc-json] tests/rustdoc-json/primitives/primitive_overloading.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/primitives/primitive_overloading/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/primitives/primitive_overloading" "--deny" "warnings" "/checkout/tests/rustdoc-json/primitives/primitive_overloading.rs" "--document-private-items" "--output-format" "json" "-Zunstable-options"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `Item { id: Id("0:1:1509"), crate_id: 0, name: Some("prim"), span: Some(Span { filename: "/checkout/tests/rustdoc-json/primitives/primitive_overloading.rs", begin: (15, 0), end: (15, 8) }), visibility: Crate, docs: Some("This is the built-in type `usize`."), links: {}, attrs: ["#[doc(primitive = \"usize\")]"], deprecation: None, inner: Module(Module { is_crate: false, items: [], is_stripped: false }) }`,
 right: `Item { id: Id("0:1:1509"), crate_id: 0, name: Some("usize"), span: Some(Span { filename: "/checkout/tests/rustdoc-json/primitives/primitive_overloading.rs", begin: (15, 0), end: (16, 1) }), visibility: Public, docs: Some("This is the built-in type `usize`."), links: {}, attrs: ["#[doc(primitive = \"usize\")]"], deprecation: None, inner: Primitive(Primitive { name: "usize", impls: [] }) }`', src/librustdoc/json/mod.rs:205:21
stack backtrace:
   0:     0x7f793b4137c5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hbac935b173aadb3a
   1:     0x7f793b482238 - core::fmt::write::h1eb4944cb33f2882
   2:     0x7f793b405621 - std::io::Write::write_fmt::h0ce08a1aaa848f1c
   3:     0x7f793b4135d1 - std::sys_common::backtrace::print::hec13032e8bc2908b
   4:     0x7f793b4169b4 - std::panicking::default_hook::{{closure}}::h23c8903f1ebfbc20
   5:     0x7f793b41667a - std::panicking::default_hook::h5607b46e715740f1
   6:     0x7f793bea2a02 - rustc_driver_impl[3ce7288b73a095af]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f793b417121 - std::panicking::rust_panic_with_hook::h406cc5481acd8c76
   8:     0x7f793b416e89 - std::panicking::begin_panic_handler::{{closure}}::hb972beadc66762c4
   9:     0x7f793b413ce4 - std::sys_common::backtrace::__rust_end_short_backtrace::h7d583e293af541d6
  10:     0x7f793b416b32 - rust_begin_unwind
  11:     0x7f793b3c5fe3 - core::panicking::panic_fmt::hbf88ff3d86589c2a
  12:     0x7f793b3c636c - core::panicking::assert_failed_inner::h02f03630400a70e2
  13:     0x5568dd6669db - core[41928c0aac29c7ca]::panicking::assert_failed::<rustdoc_json_types[5a3aaa2edde198c7]::Item, rustdoc_json_types[5a3aaa2edde198c7]::Item>
  14:     0x5568dd95ad22 - <rustdoc[371efc1770f47bdc]::json::JsonRenderer as rustdoc[371efc1770f47bdc]::formats::renderer::FormatRenderer>::item
  15:     0x5568dd95602d - <rustdoc[371efc1770f47bdc]::json::JsonRenderer as rustdoc[371efc1770f47bdc]::formats::renderer::FormatRenderer>::item
  16:     0x5568dd87b45a - rustdoc[371efc1770f47bdc]::formats::renderer::run_format::<rustdoc[371efc1770f47bdc]::json::JsonRenderer>
  17:     0x5568dd6f9f62 - rustdoc[371efc1770f47bdc]::run_renderer::<rustdoc[371efc1770f47bdc]::json::JsonRenderer>
  18:     0x5568dd78cfaa - <rustc_session[37207aac76d55711]::session::Session>::time::<core[41928c0aac29c7ca]::result::Result<(), rustc_errors[d48c189156a9466e]::ErrorGuaranteed>, rustdoc[371efc1770f47bdc]::main_args::{closure#1}::{closure#0}::{closure#1}::{closure#2}>
  19:     0x5568dd746aca - <rustc_interface[dfcd18a98e44cfa3]::passes::QueryContext>::enter::<rustdoc[371efc1770f47bdc]::main_args::{closure#1}::{closure#0}::{closure#1}, core[41928c0aac29c7ca]::result::Result<(), rustc_errors[d48c189156a9466e]::ErrorGuaranteed>>
  20:     0x5568dd691b7e - <rustc_interface[dfcd18a98e44cfa3]::queries::QueryResult<rustc_interface[dfcd18a98e44cfa3]::passes::QueryContext>>::enter::<core[41928c0aac29c7ca]::result::Result<(), rustc_errors[d48c189156a9466e]::ErrorGuaranteed>, rustdoc[371efc1770f47bdc]::main_args::{closure#1}::{closure#0}::{closure#1}>
  21:     0x5568dd9a1c18 - <rustc_interface[dfcd18a98e44cfa3]::interface::Compiler>::enter::<rustdoc[371efc1770f47bdc]::main_args::{closure#1}::{closure#0}, core[41928c0aac29c7ca]::result::Result<(), rustc_errors[d48c189156a9466e]::ErrorGuaranteed>>
  22:     0x5568dd78c67f - rustc_span[2a8bcfbda609e457]::with_source_map::<core[41928c0aac29c7ca]::result::Result<(), rustc_errors[d48c189156a9466e]::ErrorGuaranteed>, rustc_interface[dfcd18a98e44cfa3]::interface::run_compiler<core[41928c0aac29c7ca]::result::Result<(), rustc_errors[d48c189156a9466e]::ErrorGuaranteed>, rustdoc[371efc1770f47bdc]::main_args::{closure#1}>::{closure#0}::{closure#0}>
  23:     0x5568dd6945d2 - <scoped_tls[c069954ac9a39f55]::ScopedKey<rustc_span[2a8bcfbda609e457]::SessionGlobals>>::set::<rustc_interface[dfcd18a98e44cfa3]::interface::run_compiler<core[41928c0aac29c7ca]::result::Result<(), rustc_errors[d48c189156a9466e]::ErrorGuaranteed>, rustdoc[371efc1770f47bdc]::main_args::{closure#1}>::{closure#0}, core[41928c0aac29c7ca]::result::Result<(), rustc_errors[d48c189156a9466e]::ErrorGuaranteed>>
  24:     0x5568dd7b1d29 - std[fa3f5f08abe1a621]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[dfcd18a98e44cfa3]::util::run_in_thread_pool_with_globals<rustc_interface[dfcd18a98e44cfa3]::interface::run_compiler<core[41928c0aac29c7ca]::result::Result<(), rustc_errors[d48c189156a9466e]::ErrorGuaranteed>, rustdoc[371efc1770f47bdc]::main_args::{closure#1}>::{closure#0}, core[41928c0aac29c7ca]::result::Result<(), rustc_errors[d48c189156a9466e]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[41928c0aac29c7ca]::result::Result<(), rustc_errors[d48c189156a9466e]::ErrorGuaranteed>>
  25:     0x5568dd753128 - std[fa3f5f08abe1a621]::panicking::try::<core[41928c0aac29c7ca]::result::Result<(), rustc_errors[d48c189156a9466e]::ErrorGuaranteed>, core[41928c0aac29c7ca]::panic::unwind_safe::AssertUnwindSafe<<std[fa3f5f08abe1a621]::thread::Builder>::spawn_unchecked_<rustc_interface[dfcd18a98e44cfa3]::util::run_in_thread_pool_with_globals<rustc_interface[dfcd18a98e44cfa3]::interface::run_compiler<core[41928c0aac29c7ca]::result::Result<(), rustc_errors[d48c189156a9466e]::ErrorGuaranteed>, rustdoc[371efc1770f47bdc]::main_args::{closure#1}>::{closure#0}, core[41928c0aac29c7ca]::result::Result<(), rustc_errors[d48c189156a9466e]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[41928c0aac29c7ca]::result::Result<(), rustc_errors[d48c189156a9466e]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  26:     0x5568dd976d57 - <<std[fa3f5f08abe1a621]::thread::Builder>::spawn_unchecked_<rustc_interface[dfcd18a98e44cfa3]::util::run_in_thread_pool_with_globals<rustc_interface[dfcd18a98e44cfa3]::interface::run_compiler<core[41928c0aac29c7ca]::result::Result<(), rustc_errors[d48c189156a9466e]::ErrorGuaranteed>, rustdoc[371efc1770f47bdc]::main_args::{closure#1}>::{closure#0}, core[41928c0aac29c7ca]::result::Result<(), rustc_errors[d48c189156a9466e]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[41928c0aac29c7ca]::result::Result<(), rustc_errors[d48c189156a9466e]::ErrorGuaranteed>>::{closure#1} as core[41928c0aac29c7ca]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  27:     0x7f793b423fee - std::sys::unix::thread::Thread::new::thread_start::hffdfd3208ee78dbf
  28:     0x7f793b0adb43 - <unknown>
  29:     0x7f793b13fa00 - <unknown>
  30:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

