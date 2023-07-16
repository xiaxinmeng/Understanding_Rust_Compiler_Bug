
failures:

---- [pretty] pretty/conflicting-repr-hints.rs stdout ----

error: pretty-printed source does not match expected source

expected:
------------------------------------------
// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(dead_code)]

#[repr(C)]
enum A { A, }

#[repr(u64)]
enum B { B, }

 //~ WARNING conflicting representation hints
#[repr(C, u64)]
enum C { C, }

 //~ WARNING conflicting representation hints
#[repr(u32, u64)]
enum D { D, }

#[repr(C, packed)]
struct E(i32);

fn main() { }

------------------------------------------
actual:
------------------------------------------
// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(dead_code)]

#[repr(C)]
enum A { A, }

#[repr(u64)]
enum B { B, }

//~ WARNING conflicting representation hints
#[repr(C, u64)]
enum C { C, }

//~ WARNING conflicting representation hints
#[repr(u32, u64)]
enum D { D, }

#[repr(C, packed)]
struct E(i32);

fn main() { }

------------------------------------------
