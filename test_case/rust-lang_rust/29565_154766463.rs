 rust
~ ❯ multirust run nightly rustc test.rs --pretty expanded -Z unstable-options
#![feature(no_std, prelude_import)]
#![no_std]
#![deny(unused_results)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std as std;
pub struct Error {
    code: i32,
    msg: &'static str,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for Error {
    fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Error { code: ref __self_0_0, msg: ref __self_0_1 } => {
                let mut builder = __arg_0.debug_struct("Error");
                builder.field("code", &&(*__self_0_0));
                builder.field("msg", &&(*__self_0_1));
                builder.finish()
            }
        }
    }
}

fn main() { }
~ ❯ multirust run nightly rustc test.rs --pretty expanded -Z unstable-options | multirust run nightly rustc -
<anon>:5:5: 5:25 warning: unused import, #[warn(unused_imports)] on by default
<anon>:5 use std::prelude::v1::*;
             ^~~~~~~~~~~~~~~~~~~~
<anon>:19:17: 19:56 error: unused result
<anon>:19                 builder.field("code", &&(*__self_0_0));
                          ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
<anon>:3:9: 3:23 note: lint level defined here
<anon>:3 #![deny(unused_results)]
                 ^~~~~~~~~~~~~~
<anon>:20:17: 20:55 error: unused result
<anon>:20                 builder.field("msg", &&(*__self_0_1));
                          ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
<anon>:3:9: 3:23 note: lint level defined here
<anon>:3 #![deny(unused_results)]
                 ^~~~~~~~~~~~~~
error: aborting due to 2 previous errors
