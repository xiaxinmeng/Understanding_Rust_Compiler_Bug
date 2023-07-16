
time: 0.001     parsing
time: 0.000     recursion limit
time: 0.000     configuration 1
time: 0.000     gated macro checking
time: 0.000     crate injection
time: 0.009     macro loading
time: 0.000     plugin loading
time: 0.000     plugin registration
time: 0.004     expansion
time: 0.000     complete gated feature checking 1
time: 0.000     configuration 2
time: 0.000     maybe building test harness
time: 0.000     prelude injection
time: 0.000     checking that all macro invocations are gone
time: 0.000     complete gated feature checking 2
time: 0.000     assigning node ids and indexing ast
time: 0.000     external crate/lib resolution
time: 0.000     language item collection
time: 0.006     resolution
time: 0.000     lifetime resolution
time: 0.000     looking for entry point
time: 0.000     looking for plugin registrar
time: 0.000     region resolution
time: 0.000     loop checking
time: 0.000     static item recursion checking
time: 0.000     type collecting
time: 0.000     variance inference
time: 0.214     coherence checking
time: 0.105     type checking
time: 0.023     const checking
time: 0.000     privacy checking
time: 0.000     stability index
time: 0.000     intrinsic checking
time: 0.000     effect checking
time: 0.001     match checking
time: 0.000     liveness checking
time: 0.011     borrow checking
time: 0.008     rvalue checking
time: 0.000     reachability checking
time: 0.000     death checking
time: 0.011     stability checking
time: 0.000     unused lib feature checking
lib.rs:121:1: 123:2 warning: function is never used: `main`, #[warn(dead_code)] on by default
lib.rs:121 fn main() {
lib.rs:122     parse_sexp(&mut b"()".iter().cloned().peekable()).unwrap();
lib.rs:123 }
time: 0.014     lint checking
time: 0.000     resolving dependency formats
lib.rs:1:1: 1:1 error: overflow evaluating the requirement `_ : core::marker::Sized` [E0275]
lib.rs:1 use std::iter::Peekable;
         ^
lib.rs:1:1: 1:1 note: consider adding a `#![recursion_limit="128"]` attribute to your crate
lib.rs:1 use std::iter::Peekable;
         ^
error: aborting due to previous error
