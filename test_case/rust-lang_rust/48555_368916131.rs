
[01:21:57] error[E0599]: no method named `rotate` found for type `&mut std::boxed::Box<[u8]>` in the current scope
[01:21:57]     --> /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.6/src/runtest.rs:2614:30
[01:21:57]      |
[01:21:57] 2614 |                         tail.rotate(data.len());
[01:21:57]      |                              ^^^^^^
[01:21:57] 
[01:21:57] error: aborting due to previous error
[01:21:57] 
[01:21:57] If you want more information on this error, try using "rustc --explain E0599"
[01:21:57] error: Could not compile `compiletest_rs`.
