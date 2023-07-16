plain
---- src/io/mod.rs - io::Write::write_vectored (line 1423) stdout ----
error[E0433]: failed to resolve: use of undeclared type `IoSlice`
 --> src/io/mod.rs:1430:25
  |
9 |     let mut io_slice1 = IoSlice::new(&mut data1);
  |
help: consider importing this struct
  |
3 | use std::io::IoSlice;
3 | use std::io::IoSlice;
  |

error[E0433]: failed to resolve: use of undeclared type `IoSlice`
  --> src/io/mod.rs:1431:25
   |
10 |     let mut io_slice2 = IoSlice::new(&mut data2);
   |
help: consider importing this struct
   |
3  | use std::io::IoSlice;
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "std" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:18:49
