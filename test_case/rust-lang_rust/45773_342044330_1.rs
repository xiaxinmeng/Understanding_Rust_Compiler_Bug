
[01:22:25] ---- [pretty] run-pass/range_inclusive_gate.rs stdout ----
[01:22:25] 	
[01:22:25] error: pretty-printing failed in round 1 revision None
[01:22:25] status: exit code: 101
[01:22:25] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "-" "-Zunstable-options" "--unpretty" "normal" "--target" "x86_64-unknown-linux-gnu" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/range_inclusive_gate.stage2-x86_64-unknown-linux-gnu.pretty.aux" "-Crpath" "-O" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers"
[01:22:25] stdout:
[01:22:25] ------------------------------------------
[01:22:25] // Copyright 2016 The Rust Project Developers. See the COPYRIGHT
[01:22:25] // file at the top-level directory of this distribution and at
[01:22:25] // http://rust-lang.org/COPYRIGHT.
[01:22:25] //
[01:22:25] // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
[01:22:25] // http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
[01:22:25] // <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
[01:22:25] // option. This file may not be copied, modified, or distributed
[01:22:25] // except according to those terms.
[01:22:25] 
[01:22:25] // Test that you only need the syntax gate if you don't mention the structs.
[01:22:25] 
[01:22:25] #![feature(inclusive_range_syntax)]
[01:22:25] 
[01:22:25] fn main() {
[01:22:25]     let mut count = 0;
[01:22:25]     for i in 0_usize...10 { assert!(i >= 0 && i <= 10); count += i; }
[01:22:25]     assert_eq!(count , 55);
[01:22:25] }
[01:22:25] 
[01:22:25] 
[01:22:25] ------------------------------------------
[01:22:25] stderr:
[01:22:25] ------------------------------------------
[01:22:25] error: `...` syntax cannot be used in expressions
[01:22:25]   --> <anon>:17:21
[01:22:25]    |
[01:22:25] 17 |     for i in 0_usize...10 { assert!(i >= 0 && i <= 10); count += i; }
[01:22:25]    |                     ^^^
[01:22:25]    |
[01:22:25]    = help: Use `..` if you need an exclusive range (a < b)
[01:22:25]    = help: or `..=` if you need an inclusive range (a <= b)
[01:22:25] 
[01:22:25] error: aborting due to previous error
