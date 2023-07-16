
   Compiling test_crate v0.1.0 (file:///C:/Dev/Projects/crate_level_proc_macro/test_crate)
thread '<unnamed>' panicked at 'proc_macro::__internal::with_sess() called before set_parse_sess()!', libproc_macro\lib.rs:851:9
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::windows::backtrace::unwind_backtrace
             at C:\Dev\Projects\rust\src\libstd\sys\windows\backtrace\mod.rs:65
   1: std::sys_common::backtrace::_print
             at C:\Dev\Projects\rust\src\libstd\sys_common\backtrace.rs:71
   2: std::sys_common::backtrace::print
             at C:\Dev\Projects\rust\src\libstd\sys_common\backtrace.rs:58
   3: std::panicking::default_hook::{{closure}}
             at C:\Dev\Projects\rust\src\libstd\panicking.rs:380
   4: std::panicking::default_hook
             at C:\Dev\Projects\rust\src\libstd\panicking.rs:396
   5: std::panicking::rust_panic_with_hook
             at C:\Dev\Projects\rust\src\libstd\panicking.rs:576
   6: std::panicking::begin_panic<str*>
             at C:\Dev\Projects\rust\src\libstd\panicking.rs:537
   7: proc_macro::__internal::with_sess
             at C:\Dev\Projects\rust\src\libproc_macro\lib.rs:851
   8: proc_macro::TokenTree::from_internal
             at C:\Dev\Projects\rust\src\libproc_macro\lib.rs:682
   9: proc_macro::{{impl}}::next
             at C:\Dev\Projects\rust\src\libproc_macro\lib.rs:570
  10: proc_macro2::imp::{{impl}}::next
             at C:\Users\Rantanen\.cargo\registry\src\github.com-1ecc6299db9ec823\proc-macro2-0.2.2\src\unstable.rs:117
  11: proc_macro2::{{impl}}::next
             at C:\Users\Rantanen\.cargo\registry\src\github.com-1ecc6299db9ec823\proc-macro2-0.2.2\src\lib.rs:326
  12: syn::buffer::TokenBuffer::inner_new
             at C:\Users\Rantanen\.cargo\registry\src\github.com-1ecc6299db9ec823\syn-0.12.12\src\buffer.rs:176
  13: syn::buffer::TokenBuffer::new2
             at C:\Users\Rantanen\.cargo\registry\src\github.com-1ecc6299db9ec823\syn-0.12.12\src\buffer.rs:228
  14: syn::synom::{{impl}}::parse2<fn(syn::buffer::Cursor) -> core::result::Result<(crate_level_proc_macro::Crate, syn::buffer::Cursor), syn::error::ParseError>,crate_level_proc_macro::Crate>
             at C:\Users\Rantanen\.cargo\registry\src\github.com-1ecc6299db9ec823\syn-0.12.12\src\synom.rs:221
  15: syn::parse2<crate_level_proc_macro::Crate>
             at C:\Users\Rantanen\.cargo\registry\src\github.com-1ecc6299db9ec823\syn-0.12.12\src\lib.rs:601
  16: syn::parse<crate_level_proc_macro::Crate>
             at C:\Users\Rantanen\.cargo\registry\src\github.com-1ecc6299db9ec823\syn-0.12.12\src\lib.rs:580
  17: crate_level_proc_macro::attribute
             at C:\Dev\Projects\crate_level_proc_macro\src\lib.rs:54
  18: std::panicking::try::do_call
  19: _rust_maybe_catch_panic
  20: <std::thread::local::LocalKey<T>>::with
  21: <syntax_ext::proc_macro_impl::AttrProcMacro as syntax::ext::base::AttrProcMacro>::expand
  22: syntax::ext::expand::MacroExpander::expand
  23: syntax::ext::expand::MacroExpander::expand
  24: syntax::ext::expand::MacroExpander::expand_crate
  25: rustc_driver::driver::count_nodes
  26: rustc_driver::driver::count_nodes
  27: rustc_driver::driver::compile_input
  28: rustc_driver::run_compiler
error: custom attribute panicked
 --> src\main.rs:2:1
  |
2 | #![::crate_level_proc_macro::attribute]
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = help: message: proc_macro::__internal::with_sess() called before set_parse_sess()!
