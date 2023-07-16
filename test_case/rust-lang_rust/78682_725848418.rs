
$ git log --oneline src/test/codegen/vec-iter-collect-len.rs
2a663555ddf Remove licenses
1001b2beee5 inore some codegen tests when debug assertions are enabled
f67453729c1 std: Ensure OOM is classified as `nounwind`
$ git show f67453729c1 src/test/codegen/vec-iter-collect-len.rs
commit f67453729c19b435686c94936d8145051e7f1284
Author: Alex Crichton <alex@alexcrichton.com>
Date:   Thu May 24 12:03:05 2018 -0700

    std: Ensure OOM is classified as `nounwind`
    
    OOM can't unwind today, and historically it's been optimized as if it can't
    unwind. This accidentally regressed with recent changes to the OOM handler, so
    this commit adds in a codegen test to assert that everything gets optimized away
    after the OOM function is approrpiately classified as nounwind
    
    Closes #50925


added: src/test/codegen/vec-iter-collect-len.rs
────────────────────────────────────────────────────────────────────────────────────

1
// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// no-system-llvm
// compile-flags: -O
#![crate_type="lib"]

#[no_mangle]
pub fn get_len() -> usize {
    // CHECK-LABEL: @get_len
    // CHECK-NOT: call
    // CHECK-NOT: invoke
    [1, 2, 3].iter().collect::<Vec<_>>().len()
}
