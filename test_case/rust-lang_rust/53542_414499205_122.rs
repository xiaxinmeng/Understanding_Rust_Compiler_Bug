\n\nNote that the error here is in the definition of the generic function: Although\nwe only call it with a parameter th:null},{"message":"all local variables must have a statically known size","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"unsized locals are gated as an unstable feature","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0277]: the size for values of type `X` cannot be known at compilation time\n  --> /checkout/src/test/ui/unsized6.rs:50:22\n   |\nLL | fn g2<X: ?Sized + T>(x: X) {}\n   |                      ^ doesn't have a size known at compile-time\n   |\n   = help: the trait `std::marker::Sized` is not implemented for `X`\n   = note: to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>\n   = help: consider adding a `where X: std::marker::Sized` bound\n   = note: all local variables must have a statically known size\n   = help: unsized locals are gated as an unstable feature\n\n"}
[00:47:18] {"message":"aborting due to 18 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 18 previous errors\n\n"}
[00:47:18] {"message":"For more information about this error, try `rustc --explain E0277`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0277`.\n"}
[00:47:18] ------------------------------------------
[00:47:18] 
[00:47:18] thread '[ui] ui/unsized6.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3189:9
[00:47:18] 
---
151200 ./src/tools/clang
149120 ./src/llvm-emscripten/test
148688 ./obj/build/bootstrap/debug/incremental
134256 ./obj/build/bootstrap/debug/incremental/bootstrap-1v3ifugz4t07z
134252 ./obj/build/bootstrap/debug/incremental/bootstrap-1v3ifugz4t07z/s-f41k22in89-8bg0r5-1iplbejydu42a
128740 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release
127204 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu
127200 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
124436 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
