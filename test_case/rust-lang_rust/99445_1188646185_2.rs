rust
❯ cargo +stage1 expand
    Checking playground v0.0.0 (D:\git\cad97\playground)
DEBUG rustc_expand::mbe::quoted $crate changed its target crate from DefId(19:0) to DefId(0:0)
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
error[E0433]: failed to resolve: could not find `LanguageIdentifier` in the crate root
 --> src\main.rs:6:13
  |
6 |     let _ = unic_langid::langid!("en-us");
  |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in `$crate`
  |
  = note: this error originates in the macro `proc_macro_call` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider importing this struct
  |
3 | use unic_langid::LanguageIdentifier;
  |
help: if you import `LanguageIdentifier`, refer to it directly
 --> D:\.rust\cargo\registry\src\github.com-1ecc6299db9ec823\unic-langid-macros-0.9.0\src\lib.rs:4:1
4 - #[proc_macro_hack]
4 + #[proc_macro_hack]
  |
error[E0433]: failed to resolve: unresolved import
 --> src\main.rs:6:13
  |
6 |     let _ = unic_langid::langid!("en-us");
  |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in `$crate::subtags`
  |
  = note: this error originates in the macro `proc_macro_call` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider importing this struct
  |
3 | use unic_langid::subtags::Language;
  |
help: if you import `Language`, refer to it directly
 --> D:\.rust\cargo\registry\src\github.com-1ecc6299db9ec823\unic-langid-macros-0.9.0\src\lib.rs:4:1
4 - #[proc_macro_hack]
4 + #[proc_macro_hack]
  |
error[E0433]: failed to resolve: unresolved import
 --> src\main.rs:6:13
  |
6 |     let _ = unic_langid::langid!("en-us");
  |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in `$crate::subtags`
  |
  = note: this error originates in the macro `proc_macro_call` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider importing this struct
  |
3 | use unic_langid::subtags::Region;
  |
help: if you import `Region`, refer to it directly
 --> D:\.rust\cargo\registry\src\github.com-1ecc6299db9ec823\unic-langid-macros-0.9.0\src\lib.rs:4:1
4 - #[proc_macro_hack]
4 + #[proc_macro_hack]
  |
For more information about this error, try `rustc --explain E0433`.

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
            crate::LanguageIdentifier::from_raw_parts_unchecked(
                unsafe { crate::subtags::Language::from_raw_unchecked(28261u64) },
                None,
                Some(unsafe { crate::subtags::Region::from_raw_unchecked(21333u32) }),
                None,
            )
        }
    };
}
