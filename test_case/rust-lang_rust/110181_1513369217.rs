
; rustc +nightly example.rs --crate-type lib                          
error[E0433]: failed to resolve: maybe a missing crate `core`?
 --> example.rs:2:5
  |
2 | use core::{
  |     ^^^^ maybe a missing crate `core`?
  |
  = help: consider adding `extern crate core` to use the `core` crate
; rustc +nightly example.rs --crate-type lib --edition 2018 -Awarnings  # compiles fine
