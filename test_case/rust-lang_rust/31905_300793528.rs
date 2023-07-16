
error: aborting due to previous error

thread 'rustc' panicked at 'Box<Any>', /checkout/src/librustc_errors/lib.rs:457
thread 'rustc' panicked at 'couldn't compile the test', /checkout/src/librustdoc/test.rs:275

---- src/lib.rs -  (line 6) stdout ----
	error[E0514]: found crate `reed_solomon` compiled by an incompatible version of rustc
 --> <anon>:1:1
  |
1 | extern crate reed_solomon;
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = help: please recompile that crate using this compiler (rustc 1.19.0-nightly (f1140a331 2017-05-08))
  = note: crate `reed_solomon` path #1: /home/mike/dev/rust/reed-solomon-rs/target/debug/deps/libreed_solomon-52af978598229f66.rlib compiled by "rustc 1.19.0-dev (bb8d51c2e 2017-05-10)"
