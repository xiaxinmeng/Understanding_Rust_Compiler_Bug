
[01:39:41] ---- [pretty] run-pass/dyn-trait.rs stdout ----
[01:39:41] 	
[01:39:41] error: pretty-printed source does not match expected source
[01:39:41] 
[01:39:41] expected:
[01:39:41] ------------------------------------------
[01:39:41] // Copyright 2017 The Rust Project Developers. See the COPYRIGHT
[01:39:41] // file at the top-level directory of this distribution and at
[01:39:41] // http://rust-lang.org/COPYRIGHT.
[01:39:41] //
[01:39:41] // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
[01:39:41] // http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
[01:39:41] // <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
[01:39:41] // option. This file may not be copied, modified, or distributed
[01:39:41] // except according to those terms.
[01:39:41] 
[01:39:41] #![feature(dyn_trait)]
[01:39:41] 
[01:39:41] use std::fmt::Display;
[01:39:41] 
[01:39:41] static BYTE: u8 = 33;
[01:39:41] 
[01:39:41] fn main() {
[01:39:41]     let x: &(dyn 'static + Display) = &BYTE;
[01:39:41]     let y: Box<dyn Display + 'static> = Box::new(BYTE);
[01:39:41]     let _: &dyn Display = &BYTE;
[01:39:41]     let _: &dyn ::std::fmt::Display = &BYTE;
[01:39:41]     let xstr = format!("{}" , x);
[01:39:41]     let ystr = format!("{}" , y);
[01:39:41]     assert_eq!(xstr , "33");
[01:39:41]     assert_eq!(ystr , "33");
[01:39:41] }
[01:39:41] 
[01:39:41] ------------------------------------------
[01:39:41] actual:
[01:39:41] ------------------------------------------
[01:39:41] // Copyright 2017 The Rust Project Developers. See the COPYRIGHT
[01:39:41] // file at the top-level directory of this distribution and at
[01:39:41] // http://rust-lang.org/COPYRIGHT.
[01:39:41] //
[01:39:41] // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
[01:39:41] // http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
[01:39:41] // <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
[01:39:41] // option. This file may not be copied, modified, or distributed
[01:39:41] // except according to those terms.
[01:39:41] 
[01:39:41] #![feature(dyn_trait)]
[01:39:41] 
[01:39:41] use std::fmt::Display;
[01:39:41] 
[01:39:41] static BYTE: u8 = 33;
[01:39:41] 
[01:39:41] fn main() {
[01:39:41]     let x: &(dyn 'static + Display) = &BYTE;
[01:39:41]     let y: Box<dyn Display + 'static> = Box::new(BYTE);
[01:39:41]     let _: &dyn Display = &BYTE;
[01:39:41]     let _: &dyn::std::fmt::Display = &BYTE;
[01:39:41]     let xstr = format!("{}" , x);
[01:39:41]     let ystr = format!("{}" , y);
[01:39:41]     assert_eq!(xstr , "33");
[01:39:41]     assert_eq!(ystr , "33");
[01:39:41] }
[01:39:41] 
[01:39:41] ------------------------------------------
[01:39:41] 
[01:39:41] 
[01:39:41] 
[01:39:41] error: pretty-printed source does not typecheck
[01:39:41] status: exit code: 101
[01:39:41] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "-" "-Zno-trans" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/dyn-trait.pretty-out" "--target=x86_64-unknown-linux-gnu" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/dyn-trait.stage2-x86_64-unknown-linux-gnu.pretty.aux" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers"
[01:39:41] stdout:
[01:39:41] ------------------------------------------
[01:39:41] 
[01:39:41] ------------------------------------------
[01:39:41] stderr:
[01:39:41] ------------------------------------------
[01:39:41] error[E0433]: failed to resolve. Use of undeclared type or module `dyn`
[01:39:41]   --> <anon>:21:13
[01:39:41]    |
[01:39:41] 21 |     let _: &dyn::std::fmt::Display = &BYTE;
[01:39:41]    |             ^^^ Use of undeclared type or module `dyn`
[01:39:41] 
[01:39:41] error: aborting due to previous error
[01:39:41] 
[01:39:41] 
[01:39:41] ------------------------------------------
[01:39:41] 
[01:39:41] thread '[pretty] run-pass/dyn-trait.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2892:9
[01:39:41] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:39:41] 
[01:39:41] 
[01:39:41] failures:
[01:39:41]     [pretty] run-pass/dyn-trait.rs
