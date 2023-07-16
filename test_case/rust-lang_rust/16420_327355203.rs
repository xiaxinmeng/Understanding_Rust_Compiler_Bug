
error[E0425]: cannot find value `y` in this scope
  --> test.rs:5:22
   |
5  |     ($x: ident) => { y + $x }
   |                      ^ did you mean `x`?
...
10 |     foo!(x)
   |     ------- in this macro invocation

#![feature(prelude_import)]
#![no_std]
// minimal junk
#![no_std]
#[prelude_import]
use core /* 94#0 */::prelude /* 98#0 */::v1 /* 99#0 */::*;
#[macro_use]
extern crate core /* 94 */ as core /* 94#0 */;

macro_rules! foo /* 87#0 */(( $ x : ident ) => { y + $ x });

fn bar /* 91#0 */() { let x /* 88#0 */ = 1; y /* 90#2 */ + x /* 88#0 */ }
error: aborting due to previous error
