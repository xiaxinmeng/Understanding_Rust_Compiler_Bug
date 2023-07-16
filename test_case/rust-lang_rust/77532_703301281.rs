
~/cose/shaderc-test on  master ⌚ 21:04:51
$ cargo clean

~/cose/shaderc-test on  master ⌚ 21:04:57
$ rustup default stable
info: using existing install for 'stable-x86_64-unknown-linux-gnu'
info: default toolchain set to 'stable-x86_64-unknown-linux-gnu'

  stable-x86_64-unknown-linux-gnu unchanged - rustc 1.46.0 (04488afe3 2020-08-24)


~/cose/shaderc-test on  master ⌚ 21:05:14
$ cargo test
 ...

    Finished test [unoptimized + debuginfo] target(s) in 4.85s
     Running target/debug/deps/shaderc_test-e9f5c945c9e7bde8

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/inside_macro-a78288748d256321

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/out_of_macro-cdd0143a6165bf97

running 1 test
test out_of_macro ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests shaderc-test

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out


~/cose/shaderc-test on  master ⌚ 21:05:21
$ rustup default nightly
info: using existing install for 'nightly-x86_64-unknown-linux-gnu'
info: default toolchain set to 'nightly-x86_64-unknown-linux-gnu'

  nightly-x86_64-unknown-linux-gnu unchanged - rustc 1.49.0-nightly (25c8c53dd 2020-10-03)


~/cose/shaderc-test on  master ⌚ 21:05:42
$ cargo clean

~/cose/shaderc-test on  master ⌚ 21:05:46
$ cargo test 
   Compiling cc v1.0.60
   Compiling libc v0.2.78
   Compiling cmake v0.1.44
   Compiling shaderc-sys v0.6.2
   Compiling shaderc v0.6.2
   Compiling shaderc-test v0.1.0 (/mnt/void/home/archdata/cose/shaderc-test)
warning: unused import: `shaderc_test`
 --> tests/out_of_macro.rs:1:5
  |
1 | use shaderc_test;
  |     ^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

error: proc macro panicked
 --> tests/inside_macro.rs:3:1
  |
3 | test!();
  | ^^^^^^^^
  |
  = help: message: called `Result::unwrap()` on an `Err` value: CompilationError(2, "shader.glsl:3: error: \'#line\' : must by followed by an integral literal\nshader.glsl: error: shader.glsl:-1: \'#line\' : unexpected tokens following directive\n")

error: aborting due to previous error

error: could not compile `shaderc-test`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
warning: 1 warning emitted

error: build failed
