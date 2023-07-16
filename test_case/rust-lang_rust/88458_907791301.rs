plain
   Compiling chalk-derive v0.55.0
    Checking chalk-ir v0.55.0
    Checking tracing v0.1.25
    Checking rustc_index v0.0.0 (/checkout/compiler/rustc_index)
error[E0658]: use of unstable library feature 'fn_traits'
   --> compiler/rustc_index/src/vec.rs:826:5
    |
826 | /     extern "rust-call" fn call_once(self, ((n, t),): ((usize, T),)) -> Self::Output {
827 | |         (I::new(n), t)
    | |_____^
    |
    = note: see issue #29625 <https://github.com/rust-lang/rust/issues/29625> for more information
    = help: add `#![feature(fn_traits)]` to the crate attributes to enable
    = help: add `#![feature(fn_traits)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'fn_traits'
   --> compiler/rustc_index/src/vec.rs:832:5
    |
832 | /     extern "rust-call" fn call_mut(&mut self, ((n, t),): ((usize, T),)) -> Self::Output {
833 | |         (I::new(n), t)
    | |_____^
    |
    = note: see issue #29625 <https://github.com/rust-lang/rust/issues/29625> for more information
    = help: add `#![feature(fn_traits)]` to the crate attributes to enable
    = help: add `#![feature(fn_traits)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'fn_traits'
   --> compiler/rustc_index/src/vec.rs:840:5
    |
840 | /     extern "rust-call" fn call_once(self, (n,): (usize,)) -> Self::Output {
841 | |         I::new(n)
    | |_____^
    |
    = note: see issue #29625 <https://github.com/rust-lang/rust/issues/29625> for more information
    = help: add `#![feature(fn_traits)]` to the crate attributes to enable
    = help: add `#![feature(fn_traits)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'fn_traits'
   --> compiler/rustc_index/src/vec.rs:846:5
    |
846 | /     extern "rust-call" fn call_mut(&mut self, (n,): (usize,)) -> Self::Output {
847 | |         I::new(n)
    | |_____^
    |
    = note: see issue #29625 <https://github.com/rust-lang/rust/issues/29625> for more information
    = help: add `#![feature(fn_traits)]` to the crate attributes to enable
