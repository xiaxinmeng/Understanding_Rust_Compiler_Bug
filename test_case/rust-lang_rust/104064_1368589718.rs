rust
#![feature(no_core)]
#![feature(rustc_attrs)]
#![feature(rustdoc_internals)]
#![no_core]
#![rustc_coherence_is_core]

//! Link to [i32][prim@i32]

#[doc(primitive = "i32")]
mod prim_i32 {}
