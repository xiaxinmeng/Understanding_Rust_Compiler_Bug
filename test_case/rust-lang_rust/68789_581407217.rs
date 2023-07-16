
wesley@endurance:~/code/rust/rust2> cat test.rs
#![feature(core_intrinsics, const_caller_location, track_caller, const_fn)]

use std::panic::Location;
use std::intrinsics::caller_location;

type L = &'static Location<'static>;

#[track_caller]
const fn foo() -> L {
    caller_location()
}

const fn bar() -> L {
    let x: fn() -> L = foo;
    x()
}

const CTFE: L = bar();

fn main() {
    CTFE;
}

wesley@endurance:~/code/rust/rust2> rustc +stage1-2 -Z unleash-the-miri-inside-of-you test.rs
warning: skipping const checks
  --> test.rs:15:5
   |
15 |     x()
   |     ^^^

warning: path statement with no effect
  --> test.rs:21:5
   |
21 |     CTFE;
   |     ^^^^^
   |
   = note: `#[warn(path_statements)]` on by default

wesley@endurance:~/code/rust/rust2> git log -1

commit 0d34a8772251b3f9d4dd05c81d9531d455a14fc2 (HEAD, origin/master)
Merge: a2e80300cd8 a606ffdb174
Author: bors <bors@rust-lang.org>
Date:   Mon Feb 3 06:38:34 2020 +0000

    Auto merge of #68772 - matthewjasper:relate-opt, r=davidtwco
    
    Avoid exponential behaviour when relating types
    
    When equating bound types we check subtyping in both directions. Since closures are invariant in their substs, we end up comparing the two types an exponential number of times. If there are no bound variables this isn't needed.
    
    Closes #68061

