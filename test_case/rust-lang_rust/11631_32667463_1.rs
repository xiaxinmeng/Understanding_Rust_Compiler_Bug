
$ rustc trace_macros.rs
macro! { 1 2 3 4 }
macro! { 2 3 4 }
macro! { 3 4 }
macro! { 4 }
macro! {  }
println! { "hello" }
