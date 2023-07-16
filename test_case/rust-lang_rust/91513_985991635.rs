plain
   Compiling addr2line v0.16.0
error[E0433]: failed to resolve: use of undeclared crate or module `std`
    --> library/std/src/ffi/c_str.rs:1269:18
     |
1269 |         unsafe { std::mem::transmute(bytes) }
     |                  ^^^ use of undeclared crate or module `std`
error[E0433]: failed to resolve: use of undeclared crate or module `std`
    --> library/std/src/ffi/c_str.rs:1375:18
     |
     |
1375 |         unsafe { std::mem::transmute(&self.inner) }
     |                  ^^^ use of undeclared crate or module `std`
For more information about this error, try `rustc --explain E0433`.
error: could not compile `std` due to 2 previous errors
Build completed unsuccessfully in 0:00:22
