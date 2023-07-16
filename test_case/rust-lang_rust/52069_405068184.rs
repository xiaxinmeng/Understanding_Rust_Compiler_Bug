plain
[00:22:21]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:22:38]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:28:39]    Compiling rustc_typeck v0.0.0 (file:///checkout/src/librustc_typeck)
[00:28:39]    Compiling rustc_mir v0.0.0 (file:///checkout/src/librustc_mir)
[00:28:43] error[E0597]: `my_registrar` does not live long enough
[00:28:43]    --> librustc_metadata/creader.rs:582:24
[00:28:43]     |
[00:28:43] 582 |         registrar(&mut my_registrar);
[00:28:43]     |                        ^^^^^^^^^^^^ borrowed value does not live long enough
[00:28:43] 588 |     }
[00:28:43] 588 |     }
[00:28:43]     |     - `my_registrar` dropped here while still borrowed
[00:28:43]     |
[00:28:43]     = note: values in a scope are dropped in the opposite order they are created
[00:28:43] error: variable does not need to be mutable
[00:28:43]    --> librustc_metadata/creader.rs:581:13
[00:28:43]     |
[00:28:43]     |
[00:28:43] 581 |         let mut my_registrar = MyRegistrar { extensions: Vec::new(), edition: root.edition };
[00:28:43]     |             ----^^^^^^^^^^^^
[00:28:43]     |             |
[00:28:43]     |             help: remove this `mut`
[00:28:43]     = note: `-D unused-mut` implied by `-D warnings`
[00:28:43] 
[00:28:43] error: aborting due to 2 previous errors
[00:28:43] 
[00:28:43] 
[00:28:43] For more information about this error, try `rustc --explain E0597`.
[00:28:43] error: Could not compile `rustc_metadata`.
[00:28:43] 
[00:28:43] Caused by:
[00:28:43]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_metadata librustc_metadata/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=f2e516612ec0eb19 -C extra-filename=-f2e516612ec0eb19 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern flate2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libflate2-1a6e055719da9e2b.rlib --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-3aae1b07eee5f41a.rlib --extern proc_macro=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libproc_macro-02138f2ab64d4539.so --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-da4d2a69d03b2f0d.so --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-e5f576c4a2daecee.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-3575a7cf8f4898f3.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-2d5d56ac8c265ecc.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-4ea3497d1552c967.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-4ea3497d1552c967.rlib --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-fb9576173c28daa7.so --extern syntax_ext=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_ext-2d7bddb0d9db098b.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-9caed024aacf789e.so -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-00f2106305a03b52/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-11ea4ad08c115426/out` (exit code: 101)
travis_time:end:19e9f270:start=1531630179288618005,finish=1531632080027436552,duration=1900738818547

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0f8c0ac0
