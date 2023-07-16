plain
[01:00:42] 
[01:00:42] error: pretty-printed source does not match expected source
[01:00:42] expected:
[01:00:42] ------------------------------------------
[01:00:42] // Copyright 2012 The Rust Project Developers. See the COPYRIGHT
[01:00:42] // file at the top-level directory of this distribution and at
[01:00:42] // http://rust-lang.org/COPYRIGHT.
[01:00:42] //
[01:00:42] // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
[01:00:42] // http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
[01:00:42] // <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
[01:00:42] // option. This file may not be copied, modified, or distributed
[01:00:42] // except according to those terms.
[01:00:42] 
[01:00:42] // compile-flags: --crate-type=lib
[01:00:42] 
[01:00:42] // pp-exact
[01:00:42] fn f() {
[01:00:42]     The next line should not be indented.
[01:00:42] 
[01:00:42] 
[01:00:42]     That one. It shouldn't have been indented.
[01:00:42]     */
[01:00:42] }
[01:00:42] ------------------------------------------
[01:00:42] actual:
[01:00:42] ------------------------------------------
[01:00:42] ------------------------------------------
[01:00:42] // Copyright 2012 The Rust Project Developers. See the COPYRIGHT
[01:00:42] // file at the top-level directory of this distribution and at
[01:00:42] // http://rust-lang.org/COPYRIGHT.
[01:00:42] //
[01:00:42] // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
[01:00:42] // http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
[01:00:42] // <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
[01:00:42] // option. This file may not be copied, modified, or distributed
[01:00:42] // except according to those terms.
[01:00:42] 
[01:00:42] // compile-flags: --crate-type=lib
[01:00:42] 
[01:00:42] // pp-exact
[01:00:42] fn f() {
[01:00:42]      The next line should not be indented.
[01:00:42] 
[01:00:42] 
[01:00:42]      That one. It shouldn't have been indented.
[01:00:42]      */
[01:00:42] }
[01:00:42] ------------------------------------------
[01:00:42] 
[01:00:42] 
[01:00:42] thread '[pretty] pretty/block-comment-trailing-whitespace.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2020:9
[01:00:42] thread '[pretty] pretty/block-comment-trailing-whitespace.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2020:9
[01:00:42] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:00:42] 
[01:00:42] ---- [pretty] pretty/block-comment-trailing-whitespace2.rs stdout ----
[01:00:42] 
[01:00:42] error: pretty-printed source does not match expected source
[01:00:42] expected:
[01:00:42] ------------------------------------------
[01:00:42] // Copyright 2012 The Rust Project Developers. See the COPYRIGHT
[01:00:42] // file at the top-level directory of this distribution and at
[01:00:42] // http://rust-lang.org/COPYRIGHT.
[01:00:42] //
[01:00:42] // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
[01:00:42] // http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
[01:00:42] // <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
[01:00:42] // option. This file may not be copied, modified, or distributed
[01:00:42] // except according to those terms.
[01:00:42] 
[01:00:42] // compile-flags: --crate-type=lib
[01:00:42] 
[01:00:42] // pp-exact
[01:00:42] fn f() {
[01:00:42] } /*
[01:00:42]   The next line should not be indented.
[01:00:42] 
[01:00:42]   That one. It shouldn't have been indented.
[01:00:42]   */
[01:00:42] ------------------------------------------
[01:00:42] actual:
[01:00:42] ------------------------------------------
[01:00:42] ------------------------------------------
[01:00:42] // Copyright 2012 The Rust Project Developers. See the COPYRIGHT
[01:00:42] // file at the top-level directory of this distribution and at
[01:00:42] // http://rust-lang.org/COPYRIGHT.
[01:00:42] //
[01:00:42] // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
[01:00:42] // http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
[01:00:42] // <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
[01:00:42] // option. This file may not be copied, modified, or distributed
[01:00:42] // except according to those terms.
[01:00:42] 
[01:00:42] // compile-flags: --crate-type=lib
[01:00:42] 
[01:00:42] // pp-exact
[01:00:42] fn f() {
[01:00:42] } /*
[01:00:42]    The next line should not be indented.
[01:00:42] 
[01:00:42]    That one. It shouldn't have been indented.
[01:00:42]    */
[01:00:42] ------------------------------------------
[01:00:42] 
[01:00:42] 
[01:00:42] thread '[pretty] pretty/block-comment-trailing-whitespace2.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2020:9
[01:00:42] thread '[pretty] pretty/block-comment-trailing-whitespace2.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2020:9
[01:00:42] 
[01:00:42] ---- [pretty] pretty/block-comment-wchar.rs stdout ----
[01:00:42] 
[01:00:42] error: pretty-printed source does not match expected source
[01:00:42] expected:
[01:00:42] ------------------------------------------
[01:00:42] // Copyright 2013-2014 The Rust Project Developers. See the COPYRIGHT
[01:00:42] // file at the top-level directory of this distribution and at
[01:00:42] // http://rust-lang.org/COPYRIGHT.
[01:00:42] //
[01:00:42] // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
[01:00:42] // http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
[01:00:42] // <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
[01:00:42] // option. This file may not be copied, modified, or distributed
[01:00:42] // except according to those terms.
[01:00:42] 
[01:00:42] // This is meant as a test case for Issue 3961.
[01:00:42] //
[01:00:42] // Test via: rustc --pretty normal src/test/pretty/block-comment-wchar.rs
[01:00:42] // ignore-tidy-cr
[01:00:42] // ignore-tidy-tab
[01:00:42] // pp-exact:block-comment-wchar.pp
[01:00:42] fn f() {
[01:00:42]     fn nested() {
[01:00:42]           Spaced2
[01:00:42]         */
[01:00:42]         /*
[01:00:42]           Spaced10
---
[01:00:42]           CR8+2
[01:00:42]         */
[01:00:42]     }
[01:00:42]     /*
[01:00:42]       Spaced2:                       (prefixed so start of space aligns with comment)
[01:00:42]     */
[01:00:42]     /*
[01:00:42]       Tabbed2: (more indented b/c *start* of space will align with comment)
[01:00:42]     */
[01:00:42]       Spaced6:                       (Alignment removed and realigning spaces inserted)
[01:00:42]     */
[01:00:42]     /*
[01:00:42]     /*
[01:00:42]       Tabbed4+2:                     (Alignment removed and realigning spaces inserted)
[01:00:42]     */
[01:00:42]     /*
[01:00:42]     /*
[01:00:42]       VT4+2:                         (should align)
[01:00:42]     */
[01:00:42]     /*
[01:00:42]       FF4+2:                         (should align)
[01:00:42]     */
[01:00:42]     /*
[01:00:42]       CR4+2:                         (should align)
[01:00:42]     */
[01:00:42]     /*
[01:00:42]       NEL4+2:                        (should align)
[01:00:42]     */
[01:00:42]     /*
[01:00:42]       Ogham Space Mark 4+2:          (should align)
[01:00:42]     */
[01:00:42]     /*
[01:00:42]       Ogham Space Mark 4+2: (should align)
[01:00:42]     */
[01:00:42]     /*
[01:00:42]       Four-per-em space 4+2:         (should align)
[01:00:42]     */
[01:00:42]     /*
[01:00:42]     /*
[01:00:42]       Ogham Space Mark   count 1: (should align)
[01:00:42]       Ogham Space Mark   count 2: (should align)
[01:00:42]       Ogham Space Mark   count 3: (should align)
[01:00:42]       Ogham Space Mark   count 4: (should align)
[01:00:42]       Ogham Space Mark   count 5: (should align)
[01:00:42]       Ogham Space Mark   count 6: (should align)
[01:00:42]       Ogham Space Mark   count 7: (should align)
[01:00:42]       Ogham Space Mark   count 8: (should align)
[01:00:42]       Ogham Space Mark   count 9: (should align)
[01:00:42]       Ogham Space Mark   count A: (should align)
[01:00:42]       Ogham Space Mark   count B: (should align)
[01:00:42]       Ogham Space Mark   count C: (should align)
[01:00:42]       Ogham Space Mark   count D: (should align)
[01:00:42]       Ogham Space Mark   count E: (should align)
[01:00:42]       Ogham Space Mark   count F: (should align)
[01:00:42]     */
[01:00:42] 
[01:00:42] 
[01:00:42]     /* */
[01:00:42] 
[01:00:42] 
[01:00:42]     /*
[01:00:42]       Hello from offset 6
[01:00:42]       Space 6+2:                     compare A
[01:00:42]       Ogham Space Mark 6+2: compare B
[01:00:42]     */
[01:00:42]     /* */
[01:00:42] 
[01:00:42]     /*
[01:00:42]     /*
[01:00:42]       Hello from another offset 6 with wchars establishing column offset
[01:00:42]       Space 6+2:                     compare C
[01:00:42]       Ogham Space Mark 6+2: compare D
[01:00:42]     */
[01:00:42] }
[01:00:42] fn main() {
[01:00:42] fn main() {
[01:00:42]     // Taken from http://www.unicode.org/Public/UNIDATA/PropList.txt
[01:00:42]     let chars =
[01:00:42]         ['\x0A', '\x0B', '\x0C', '\x0D', '\x20', '\u{85}', '\u{A0}',
[01:00:42]          '\u{1680}', '\u{2000}', '\u{2001}', '\u{2002}', '\u{2003}',
[01:00:42]          '\u{2004}', '\u{2005}', '\u{2006}', '\u{2007}', '\u{2008}',
[01:00:42]          '\u{2009}', '\u{200A}', '\u{2028}', '\u{2029}', '\u{202F}',
[01:00:42]          '\u{205F}', '\u{3000}'];
[01:00:42]     for c in &chars {
[01:00:42]         let ws = c.is_whitespace();
[01:00:42]         println!("{} {}" , c , ws);
[01:00:42] }
[01:00:42] 
[01:00:42] ------------------------------------------
[01:00:42] actual:
[01:00:42] actual:
[01:00:42] ------------------------------------------
[01:00:42] // Copyright 2013-2014 The Rust Project Developers. See the COPYRIGHT
[01:00:42] // file at the top-level directory of this distribution and at
[01:00:42] // http://rust-lang.org/COPYRIGHT.
[01:00:42] //
[01:00:42] // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
[01:00:42] // http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
[01:00:42] // <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
[01:00:42] // option. This file may not be copied, modified, or distributed
[01:00:42] // except according to those terms.
[01:00:42] 
[01:00:42] // This is meant as a test case for Issue 3961.
[01:00:42] //
[01:00:42] // Test via: rustc --pretty normal src/test/pretty/block-comment-wchar.rs
[01:00:42] // ignore-tidy-cr
[01:00:42] // ignore-tidy-tab
[01:00:42] // pp-exact:block-comment-wchar.pp
[01:00:42] fn f() {
[01:00:42]     fn nested() {
[01:00:42]           Spaced2
[01:00:42]          */
[01:00:42]         /*
[01:00:42]            Spaced10
---
[01:00:42]           CR8+2
[01:00:42]          */
[01:00:42]     }
[01:00:42]     /*
[01:00:42]       Spaced2:                       (prefixed so start of space aligns with comment)
[01:00:42]      */
[01:00:42]     /*
[01:00:42]       Tabbed2: (more indented b/c *start* of space will align with comment)
[01:00:42]      */
[01:00:42]        Spaced6:                       (Alignment removed and realigning spaces inserted)
[01:00:42]      */
[01:00:42]     /*
[01:00:42]     /*
[01:00:42]        Tabbed4+2:                     (Alignment removed and realigning spaces inserted)
[01:00:42]      */
[01:00:42]     /*
[01:00:42]     /*
[01:00:42]        VT4+2:                         (should align)
[01:00:42]      */
[01:00:42]     /*
[01:00:42]        FF4+2:                         (should align)
[01:00:42]      */
[01:00:42]     /*
[01:00:42]       CR4+2:                         (should align)
[01:00:42]      */
[01:00:42]     /*
[01:00:42]        NEL4+2:                        (should align)
[01:00:42]      */
[01:00:42]     /*
[01:00:42]        Ogham Space Mark 4+2:          (should align)
[01:00:42]      */
[01:00:42]     /*
[01:00:42]        Ogham Space Mark 4+2: (should align)
[01:00:42]      */
[01:00:42]     /*
[01:00:42]        Four-per-em space 4+2:         (should align)
[01:00:42]      */
[01:00:42]     /*
[01:00:42]     /*
[01:00:42]        Ogham Space Mark   count 1: (should align)
[01:00:42]        Ogham Space Mark   count 2: (should align)
[01:00:42]        Ogham Space Mark   count 3: (should align)
[01:00:42]        Ogham Space Mark   count 4: (should align)
[01:00:42]        Ogham Space Mark   count 5: (should align)
[01:00:42]        Ogham Space Mark   count 6: (should align)
[01:00:42]        Ogham Space Mark   count 7: (should align)
[01:00:42]        Ogham Space Mark   count 8: (should align)
[01:00:42]        Ogham Space Mark   count 9: (should align)
[01:00:42]        Ogham Space Mark   count A: (should align)
[01:00:42]        Ogham Space Mark   count B: (should align)
[01:00:42]        Ogham Space Mark   count C: (should align)
[01:00:42]        Ogham Space Mark   count D: (should align)
[01:00:42]        Ogham Space Mark   count E: (should align)
[01:00:42]        Ogham Space Mark   count F: (should align)
[01:00:42]      */
[01:00:42] 
[01:00:42] 
[01:00:42]     /* */
[01:00:42] 
[01:00:42] 
[01:00:42]     /*
[01:00:42]        Hello from offset 6
[01:00:42]        Space 6+2:                     compare A
[01:00:42]        Ogham Space Mark 6+2: compare B
[01:00:42]      */
[01:00:42]     /* */
[01:00:42] 
[01:00:42]     /*
[01:00:42]     /*
[01:00:42]        Hello from another offset 6 with wchars establishing column offset
[01:00:42]        Space 6+2:                     compare C
[01:00:42]        Ogham Space Mark 6+2: compare D
[01:00:42]      */
[01:00:42] }
[01:00:42] fn main() {
[01:00:42] fn main() {
[01:00:42]     // Taken from http://www.unicode.org/Public/UNIDATA/PropList.txt
[01:00:42]     let chars =
[01:00:42]         ['\x0A', '\x0B', '\x0C', '\x0D', '\x20', '\u{85}', '\u{A0}',
[01:00:42]          '\u{1680}', '\u{2000}', '\u{2001}', '\u{2002}', '\u{2003}',
[01:00:42]          '\u{2004}', '\u{2005}', '\u{2006}', '\u{2007}', '\u{2008}',
[01:00:42]          '\u{2009}', '\u{200A}', '\u{2028}', '\u{2029}', '\u{202F}',
[01:00:42]          '\u{205F}', '\u{3000}'];
[01:00:42]     for c in &chars {
[01:00:42]         let ws = c.is_whitespace();
[01:00:42]         println!("{} {}" , c , ws);
[01:00:42] }
[01:00:42] 
[01:00:42] ------------------------------------------
[01:00:42] 
---
[01:00:42] 
[01:00:42] error: pretty-printed source does not match expected source
[01:00:42] expected:
[01:00:42] ------------------------------------------
[01:00:42] // Copyright 2015 The Rust Project Developers. See the COPYRIGHT
[01:00:42] // file at the top-level directory of this distribution and at
[01:00:42] // http://rust-lang.org/COPYRIGHT.
[01:00:42] //
[01:00:42] // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
[01:00:42] // http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
[01:00:42] // <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
[01:00:42] // option. This file may not be copied, modified, or distributed
[01:00:42] // except according to those terms.
[01:00:42] 
[01:00:42] // pp-exact
[01:00:42] 
[01:00:42] #![feature(custom_attribute)]
[01:00:42] #![feature(box_syntax)]
[01:00:42] #![feature(stmt_expr_attributes)]
[01:00:42] fn main() { }
[01:00:42] 
[01:00:42] 
[01:00:42] fn _0() {
[01:00:42]     #[attr]
[01:00:42]     foo();
[01:00:42] }
[01:00:42] 
[01:00:42] 
[01:00:42] fn _1() {
[01:00:42]     #[attr]
[01:00:42]     unsafe {
[01:00:42]         // code
[01:00:42]     }
[01:00:42]     }
[01:00:42] }
[01:00:42] 
[01:00:42] fn _2() {
[01:00:42]     #[attr]
[01:00:42]     #[attr]
[01:00:42]     { foo(); }
[01:00:42]     {
[01:00:42]     {
[01:00:42]         #![attr]
[01:00:42]         foo()
[01:00:42]     }
[01:00:42] }
[01:00:42] 
[01:00:42] 
[01:00:42] fn _3() {
[01:00:42]     #[attr]
[01:00:42]     #[attr]
[01:00:42]     match () { _ => { } }
[01:00:42] }
[01:00:42] 
[01:00:42] fn _4() {
[01:00:42]     #[attr]
[01:00:42]     match () {
[01:00:42]     match () {
[01:00:42]         #![attr]
[01:00:42]     }
[01:00:42] 
[01:00:42]     let _ =
[01:00:42]     let _ =
[01:00:42]         #[attr] match () {
[01:00:42]                     #![attr]
[01:00:42]                     () => (),
[01:00:42] }
[01:00:42] 
[01:00:42] 
[01:00:42] fn _5() {
[01:00:42]     #[attr]
[01:00:42]     let x = 1;
[01:00:42] 
[01:00:42] 
[01:00:42]     let x = #[attr] 1;
[01:00:42] 
[01:00:42]     let y = ();
[01:00:42]     let z = ();
[01:00:42] 
[01:00:42]     foo3(x, #[attr] y, z);
[01:00:42] 
[01:00:42]     qux(3 + #[attr] 2);
[01:00:42] }
[01:00:42] 
[01:00:42] fn _6() {
[01:00:42]     #[attr]
[01:00:42]     #[attr]
[01:00:42]     [#![attr] 1, 2, 3];
[01:00:42] 
[01:00:42]     let _ = #[attr] [#![attr] 1, 2, 3];
[01:00:42]     #[attr]
[01:00:42]     #[attr]
[01:00:42]     [#![attr] 1; 4];
[01:00:42] 
[01:00:42]     let _ = #[attr] [#![attr] 1; 4];
[01:00:42] }
[01:00:42] struct Foo {
[01:00:42]     data: (),
[01:00:42] }
[01:00:42] 
[01:00:42] 
[01:00:42] struct Bar(());
[01:00:42] 
[01:00:42] fn _7() {
[01:00:42]     #[attr]
[01:00:42]     #[attr]
[01:00:42]     Foo{#![attr] data: (),};
[01:00:42] 
[01:00:42]     let _ = #[attr] Foo{#![attr] data: (),};
[01:00:42] }
[01:00:42] 
[01:00:42] fn _8() {
[01:00:42]     #[attr]
[01:00:42]     #[attr]
[01:00:42]     (#![attr] );
[01:00:42]     #[attr]
[01:00:42]     #[attr]
[01:00:42]     (#![attr] 0);
[01:00:42]     #[attr]
[01:00:42]     #[attr]
[01:00:42]     (#![attr] 0,);
[01:00:42]     #[attr]
[01:00:42]     #[attr]
[01:00:42]     (#![attr] 0, 1);
[01:00:42] }
[01:00:42] 
[01:00:42] fn _9() {
[01:00:42]     macro_rules! stmt_mac((  ) => { let _ = (  ) ; });
[01:00:42]     #[attr]
[01:00:42]     stmt_mac!();
[01:00:42] 
[01:00:42]     /*
[01:00:42]     /*
[01:00:42]     // pre existing pp bug: delimiter styles gets lost:
[01:00:42]     #[attr]
[01:00:42]     #[attr]
[01:00:42]     stmt_mac!{ };
[01:00:42]     #[attr]
[01:00:42]     stmt_mac![];
[01:00:42] 
[01:00:42]     #[attr]
[01:00:42]     #[attr]
[01:00:42]     stmt_mac!{ } // pre-existing pp bug: compiler ICEs with a None unwrap
[01:00:42]     */
[01:00:42]     let _ = ();
[01:00:42] }
[01:00:42] 
[01:00:42] 
[01:00:42] macro_rules! expr_mac((  ) => { (  ) });
[01:00:42] 
[01:00:42] fn _10() {
[01:00:42] 
[01:00:42]     let _ = #[attr] expr_mac!();
[01:00:42]     /*
[01:00:42]     /*
[01:00:42]     // pre existing pp bug: delimiter styles gets lost:
[01:00:42]     let _ = #[attr] expr_mac![];
[01:00:42]     let _ = #[attr] expr_mac!{};
[01:00:42]     */
[01:00:42] }
[01:00:42] 
[01:00:42] fn _11() {
[01:00:42]     let _ = #[attr] box 0;
[01:00:42]     let _: [(); 0] = #[attr] [#![attr] ];
[01:00:42]     let _ = #[attr] [#![attr] 0, 0];
[01:00:42]     let _ = #[attr] [#![attr] 0; 0];
[01:00:42]     let _ = #[attr] foo();
[01:00:42]     let _ = #[attr] 1i32.clone();
[01:00:42]     let _ = #[attr] (#![attr] );
[01:00:42]     let _ = #[attr] (#![attr] 0);
[01:00:42]     let _ = #[attr] (#![attr] 0,);
[01:00:42]     let _ = #[attr] (#![attr] 0, 0);
[01:00:42]     let _ = #[attr] 0 + #[attr] 0;
[01:00:42]     let _ = #[attr] !0;
[01:00:42]     let _ = #[attr] -0i32;
[01:00:42]     let _ = #[attr] false;
[01:00:42]     let _ = #[attr] 'c';
[01:00:42]     let _ = #[attr] 0;
[01:00:42]     let _ = #[attr] 0 as usize;
[01:00:42]     let _ =
[01:00:42]         #[attr] while false {
[01:00:42]                     #![attr]
[01:00:42]     let _ =
[01:00:42]     let _ =
[01:00:42]         #[attr] while let None = Some(()) {
[01:00:42]                     #![attr]
[01:00:42]     let _ =
[01:00:42]     let _ =
[01:00:42]         #[attr] for _ in 0..0 {
[01:00:42]                     #![attr]
[01:00:42]                 };
[01:00:42]     // FIXME: pp bug, two spaces after the loop
[01:00:42]     let _ =
[01:00:42]         #[attr] loop  {
[01:00:42]                     #![attr]
[01:00:42]     let _ =
[01:00:42]     let _ =
[01:00:42]         #[attr] match false {
[01:00:42]                     #![attr]
[01:00:42]                 };
[01:00:42]                 };
[01:00:42]     let _ = #[attr] || #[attr] ();
[01:00:42]     let _ = #[attr] move || #[attr] ();
[01:00:42]     let _ =
[01:00:42]         #[attr] ||
[01:00:42]                     {
[01:00:42]                         #![attr]
[01:00:42]                         #[attr]
[01:00:42]                         ()
[01:00:42]     let _ =
[01:00:42]     let _ =
[01:00:42]         #[attr] move ||
[01:00:42]                     {
[01:00:42]                         #![attr]
[01:00:42]                         #[attr]
[01:00:42]                         ()
[01:00:42]     let _ =
[01:00:42]         #[attr] {
[01:00:42]         #[attr] {
[01:00:42]                     #![attr]
[01:00:42]     let _ =
[01:00:42]         #[attr] {
[01:00:42]         #[attr] {
[01:00:42]                     #![attr]
[01:00:42]                     let _ = ();
[01:00:42]     let _ =
[01:00:42]         #[attr] {
[01:00:42]         #[attr] {
[01:00:42]                     #![attr]
[01:00:42]                     let _ = ();
[01:00:42]                     ()
[01:00:42]     let mut x = 0;
[01:00:42]     let mut x = 0;
[01:00:42]     let _ = #[attr] x = 15;
[01:00:42]     let _ = #[attr] x += 15;
[01:00:42]     let s = Foo{data: (),};
[01:00:42]     let _ = #[attr] s.data;
[01:00:42]     let _ = (#[attr] s).data;
[01:00:42]     let t = Bar(());
[01:00:42]     let _ = #[attr] t.0;
[01:00:42]     let _ = (#[attr] t).0;
[01:00:42]     let v = vec!(0);
[01:00:42]     let _ = #[attr] v[0];
[01:00:42]     let _ = (#[attr] v)[0];
[01:00:42]     let _ = #[attr] 0..#[attr] 0;
[01:00:42]     let _ = #[attr] 0..;
[01:00:42]     let _ = #[attr] (0..0);
[01:00:42]     let _ = #[attr] (0..);
[01:00:42]     let _ = #[attr] (..0);
[01:00:42]     let _ = #[attr] (..);
[01:00:42]     let _: fn(&u32) -> u32 = #[attr] std::clone::Clone::clone;
[01:00:42]     let _ = #[attr] &0;
[01:00:42]     let _ = #[attr] &mut 0;
[01:00:42]     let _ = #[attr] &#[attr] 0;
[01:00:42]     let _ = #[attr] &mut #[attr] 0;
[01:00:42]     // FIXME: pp bug, extra space after keyword?
[01:00:42]     while false { let _ = #[attr] continue ; }
[01:00:42]     while true { let _ = #[attr] break ; }
[01:00:42]     || #[attr] return;
[01:00:42]     let _ = #[attr] expr_mac!();
[01:00:42]     /* FIXME: pp bug, losing delimiter styles
[01:00:42]     let _ = #[attr] expr_mac![];
[01:00:42]     let _ = #[attr] expr_mac!{};
[01:00:42]     */
[01:00:42]     let _ = #[attr] Foo{#![attr] data: (),};
[01:00:42]     let _ = #[attr] Foo{#![attr] ..s};
[01:00:42]     let _ = #[attr] Foo{#![attr] data: (), ..s};
[01:00:42] make: *** [check-aux] Error 1
[01:00:42]     let _ = #[attr] (#![attr] 0);
[01:00:42] }
[01:00:42] 
[01:00:42] fn _12() {
[01:00:42]     #[attr]
[01:00:42]     let _ = 0;
[01:00:42]     #[attr]
[01:00:42]     0;
[01:00:42] 
[01:00:42]     #[attr]
[01:00:42]     #[attr]
[01:00:42]     expr_mac!();
[01:00:42] 
[01:00:42]     #[attr]
[01:00:42]     {
[01:00:42]         #![attr]
[01:00:42] }
[01:00:42] 
[01:00:42] 
[01:00:42] /////////////////
[01:00:42] fn foo() { }
[01:00:42] fn foo() { }
[01:00:42] fn foo3(_: i32, _: (), _: ()) { }
[01:00:42] fn qux(_: i32) { }
[01:00:42] ------------------------------------------
[01:00:42] actual:
[01:00:42] ------------------------------------------
[01:00:42] ------------------------------------------
[01:00:42] // Copyright 2015 The Rust Project Developers. See the COPYRIGHT
[01:00:42] // file at the top-level directory of this distribution and at
[01:00:42] // http://rust-lang.org/COPYRIGHT.
[01:00:42] //
[01:00:42] // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
[01:00:42] // http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
[01:00:42] // <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
[01:00:42] // option. This file may not be copied, modified, or distributed
[01:00:42] // except according to those terms.
[01:00:42] 
[01:00:42] // pp-exact
[01:00:42] 
[01:00:42] #![feature(custom_attribute)]
[01:00:42] #![feature(box_syntax)]
[01:00:42] #![feature(stmt_expr_attributes)]
[01:00:42] fn main() { }
[01:00:42] 
[01:00:42] 
[01:00:42] fn _0() {
[01:00:42]     #[attr]
[01:00:42]     foo();
[01:00:42] }
[01:00:42] 
[01:00:42] 
[01:00:42] fn _1() {
[01:00:42]     #[attr]
[01:00:42]     unsafe {
[01:00:42]         // code
[01:00:42]     }
[01:00:42]     }
[01:00:42] }
[01:00:42] 
[01:00:42] fn _2() {
[01:00:42]     #[attr]
[01:00:42]     #[attr]
[01:00:42]     { foo(); }
[01:00:42]     {
[01:00:42]     {
[01:00:42]         #![attr]
[01:00:42]         foo()
[01:00:42]     }
[01:00:42] }
[01:00:42] 
[01:00:42] 
[01:00:42] fn _3() {
[01:00:42]     #[attr]
[01:00:42]     #[attr]
[01:00:42]     match () { _ => { } }
[01:00:42] }
[01:00:42] 
[01:00:42] fn _4() {
[01:00:42]     #[attr]
[01:00:42]     match () {
[01:00:42]     match () {
[01:00:42]         #![attr]
[01:00:42]     }
[01:00:42] 
[01:00:42]     let _ =
[01:00:42]     let _ =
[01:00:42]         #[attr] match () {
[01:00:42]                     #![attr]
[01:00:42]                     () => (),
[01:00:42] }
[01:00:42] 
[01:00:42] 
[01:00:42] fn _5() {
[01:00:42]     #[attr]
[01:00:42]     let x = 1;
[01:00:42] 
[01:00:42] 
[01:00:42]     let x = #[attr] 1;
[01:00:42] 
[01:00:42]     let y = ();
[01:00:42]     let z = ();
[01:00:42] 
[01:00:42]     foo3(x, #[attr] y, z);
[01:00:42] 
[01:00:42]     qux(3 + #[attr] 2);
[01:00:42] }
[01:00:42] 
[01:00:42] fn _6() {
[01:00:42]     #[attr]
[01:00:42]     #[attr]
[01:00:42]     [#![attr] 1, 2, 3];
[01:00:42] 
[01:00:42]     let _ = #[attr] [#![attr] 1, 2, 3];
[01:00:42]     #[attr]
[01:00:42]     #[attr]
[01:00:42]     [#![attr] 1; 4];
[01:00:42] 
[01:00:42]     let _ = #[attr] [#![attr] 1; 4];
[01:00:42] }
[01:00:42] struct Foo {
[01:00:42]     data: (),
[01:00:42] }
[01:00:42] 
[01:00:42] 
[01:00:42] struct Bar(());
[01:00:42] 
[01:00:42] fn _7() {
[01:00:42]     #[attr]
[01:00:42]     #[attr]
[01:00:42]     Foo{#![attr] data: (),};
[01:00:42] 
[01:00:42]     let _ = #[attr] Foo{#![attr] data: (),};
[01:00:42] }
[01:00:42] 
[01:00:42] fn _8() {
[01:00:42]     #[attr]
[01:00:42]     #[attr]
[01:00:42]     (#![attr] );
[01:00:42]     #[attr]
[01:00:42]     #[attr]
[01:00:42]     (#![attr] 0);
[01:00:42]     #[attr]
[01:00:42]     #[attr]
[01:00:42]     (#![attr] 0,);
[01:00:42]     #[attr]
[01:00:42]     #[attr]
[01:00:42]     (#![attr] 0, 1);
[01:00:42] }
[01:00:42] 
[01:00:42] fn _9() {
[01:00:42]     macro_rules! stmt_mac((  ) => { let _ = (  ) ; });
[01:00:42]     #[attr]
[01:00:43]     stmt_mac!();
[01:00:43] 
[01:00:43]     /*
[01:00:43]     /*
[01:00:43]      // pre existing pp bug: delimiter styles gets lost:
[01:00:43]      #[attr]
[01:00:43]      #[attr]
[01:00:43]      stmt_mac!{ };
[01:00:43]      #[attr]
[01:00:43]      stmt_mac![];
[01:00:43] 
[01:00:43]      #[attr]
[01:00:43]      #[attr]
[01:00:43]      stmt_mac!{ } // pre-existing pp bug: compiler ICEs with a None unwrap
[01:00:43]      */
[01:00:43]     let _ = ();
[01:00:43] }
[01:00:43] 
[01:00:43] 
[01:00:43] macro_rules! expr_mac((  ) => { (  ) });
[01:00:43] 
[01:00:43] fn _10() {
[01:00:43] 
[01:00:43]     let _ = #[attr] expr_mac!();
[01:00:43]     /*
[01:00:43]     /*
[01:00:43]      // pre existing pp bug: delimiter styles gets lost:
[01:00:43]      let _ = #[attr] expr_mac![];
[01:00:43]      let _ = #[attr] expr_mac!{};
[01:00:43]      */
[01:00:43] }
[01:00:43] 
[01:00:43] fn _11() {
[01:00:43]     let _ = #[attr] box 0;
[01:00:43]     let _: [(); 0] = #[attr] [#![attr] ];
[01:00:43]     let _ = #[attr] [#![attr] 0, 0];
[01:00:43]     let _ = #[attr] [#![attr] 0; 0];
[01:00:43]     let _ = #[attr] foo();
[01:00:43]     let _ = #[attr] 1i32.clone();
[01:00:43]     let _ = #[attr] (#![attr] );
[01:00:43]     let _ = #[attr] (#![attr] 0);
[01:00:43]     let _ = #[attr] (#![attr] 0,);
[01:00:43]     let _ = #[attr] (#![attr] 0, 0);
[01:00:43]     let _ = #[attr] 0 + #[attr] 0;
[01:00:43]     let _ = #[attr] !0;
[01:00:43]     let _ = #[attr] -0i32;
[01:00:43]     let _ = #[attr] false;
[01:00:43]     let _ = #[attr] 'c';
[01:00:43]     let _ = #[attr] 0;
[01:00:43]     let _ = #[attr] 0 as usize;
[01:00:43]     let _ =
[01:00:43]         #[attr] while false {
[01:00:43]                     #![attr]
[01:00:43]     let _ =
[01:00:43]     let _ =
[01:00:43]         #[attr] while let None = Some(()) {
[01:00:43]                     #![attr]
[01:00:43]     let _ =
[01:00:43]     let _ =
[01:00:43]         #[attr] for _ in 0..0 {
[01:00:43]                     #![attr]
[01:00:43]                 };
[01:00:43]     // FIXME: pp bug, two spaces after the loop
[01:00:43]     let _ =
[01:00:43]         #[attr] loop  {
[01:00:43]                     #![attr]
[01:00:43]     let _ =
[01:00:43]     let _ =
[01:00:43]         #[attr] match false {
[01:00:43]                     #![attr]
[01:00:43]                 };
[01:00:43]                 };
[01:00:43]     let _ = #[attr] || #[attr] ();
[01:00:43]     let _ = #[attr] move || #[attr] ();
[01:00:43]     let _ =
[01:00:43]         #[attr] ||
[01:00:43]                     {
[01:00:43]                         #![attr]
[01:00:43]                         #[attr]
---
[01:00:43] test result: FAILED. 47 passed; 4 failed; 0 ignored; 0 measured; 0 filtered out
[01:00:43] 
[01:00:43] 
[01:00:43] 
[01:00:43] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/pretty" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/pretty" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "pretty" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--llvm-version" "8.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:00:43] 
[01:00:43] 
[01:00:43] 
[01:00:43] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/pretty src/test/run-pass/pretty src/test/run-fail/pretty src/test/run-pass-valgrind/pretty src/test/run-pass-fulldeps/pretty src/test/run-fail-fulldeps/pretty src/tools/cargo src/tools/cargotest
[01:00:43] Build completed unsuccessfully in 0:01:08
[01:00:43] Makefile:60: recipe for target 'check-aux' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0975bb90
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Dec  4 20:55:30 UTC 2018
---
travis_time:end:011e2b26:start=1543956931904141515,finish=1543956931911434442,duration=7292927
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:17a69078
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:089a3dd0
travis_time:start:089a3dd0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2585a168
$ dmesg | grep -i kill
