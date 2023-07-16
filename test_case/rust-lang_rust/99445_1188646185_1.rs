rust
❯ cargo +nightly expand
    Checking playground v0.0.0 (D:\git\cad97\playground)
warning: some trace filter directives would enable traces that are disabled statically
 | `rustc_expand::mbe::quoted=debug` would enable the DEBUG level for the `rustc_expand::mbe::quoted` target
 = note: the static max level is `info`
 = help: to enable DEBUG logging, remove the `max_level_info` feature
note: trace_macro
 --> src\main.rs:6:13
  |
6 |     let _ = unic_langid::langid!("en-us");
  |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: expanding `langid! { "en-us" }`
  = note: to `{
              #[derive($crate :: _proc_macro_hack_langid)] #[allow(dead_code)] enum
              ProcMacroHack { Value = (stringify! { "en-us" }, 0).1, } proc_macro_call!
              ()
          }`
  = note: expanding `proc_macro_call! {  }`
  = note: to `unsafe
          {
              $crate :: LanguageIdentifier ::
              from_raw_parts_unchecked(unsafe
              { $crate :: subtags :: Language :: from_raw_unchecked(28261u64) }, None,
              Some(unsafe
              { $crate :: subtags :: Region :: from_raw_unchecked(21333u32) }), None)
          }`
    Finished dev [unoptimized + debuginfo] target(s) in 0.14s

#![feature(prelude_import)]
#![feature(trace_macros)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
fn main() {
    let _ = {
        #[allow(dead_code)]
        enum ProcMacroHack {
            Value = ("\"en-us\"", 0).1,
        }
        unsafe {
            ::unic_langid_macros::LanguageIdentifier::from_raw_parts_unchecked(
                unsafe { ::unic_langid_macros::subtags::Language::from_raw_unchecked(28261u64) },
                None,
                Some(unsafe {
                    ::unic_langid_macros::subtags::Region::from_raw_unchecked(21333u32)
                }),
                None,
            )
        }
    };
}
