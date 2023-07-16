plain
[00:44:58] ....................................................................................................
[00:45:01] ............i.......................................................................................
[00:45:06] ..................................................................................................i.
[00:45:09] ......i.............................................................................................
[00:45:12] ............ii.iii.............................................................................F....
[00:45:17] ....................................................................................................
[00:45:19] ....................................................................................................
[00:45:21] ....................................................................................................
[00:45:23] ....................................................................................................
---
[00:46:32] ....................................................................................................
[00:46:36] ............................................i.......................................................
[00:46:39] ....................................................................................................
[00:46:42] ....................................................................................................
cals are gated as an unstable feature
[00:46:45] - For more information about this error, try `rustc --explain E0308`.
[00:46:45] - For more information about this error, try `rustc --explain E0308`.
[00:46:45] + error[E0277]: the size for values of type `dyn T` cannot be known at compilation time
[00:46:45] +   --> $DIR/elide-errors-on-mismatched-tuple.rs:24:13
[00:46:45] +    |
[00:46:45] + LL |     let (a, b, c) = (A::new(), A::new()); // This tuple is 2 elements, should be three
[00:46:45] +    |             ^ doesn't have a size known at compile-time
[00:46:45] +    |
[00:46:45] +    = help: the trait `std::marker::Sized` is not implemented for `dyn T`
[00:46:45] +    = note: all local variables must have a statically known size
[00:46:45] +    = note: all local variables must have a statically known size
[00:46:45] +    = help: unsized locals are gated as an unstable feature
[00:46:45] + 
[00:46:45] + error[E0277]: the size for values of type `dyn T` cannot be known at compilation time
[00:46:45] +   --> $DIR/elide-errors-on-mismatched-tuple.rs:24:16
[00:46:45] +    |
[00:46:45] + LL |     let (a, b, c) = (A::new(), A::new()); // This tuple is 2 elements, should be three
[00:46:45] +    |                ^ doesn't have a size known at compile-time
[00:46:45] +    |
[00:46:45] +    = help: the trait `std::marker::Sized` is not implemented for `dyn T`
[00:46:45] +    = note: to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:46:45] +    = note: all local variables must have a statically krust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"all local variables must have a statically known size","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"unsized locals are gated as an unstable feature","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0277]: the size for values of type `dyn T` cannot be known at compilation time\n  --> /checkout/src/test/ui/elide-errors-on-mismatched-tuple.rs:24:16\n   |\nLL |     let (a, b, c) = (A::new(), A::new()); // This tuple is 2 elements, should be three\n   |                ^ doesn't have a size known at compile-time\n   |\n   = help: the trait `std::marker::Sized` is not implemented for `dyn T`\n   = note: to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>\n   = note: all local variables must have a statically known size\n   = help: unsized locals are gated as an unstable feature\n\n"}
[00:46:45] {"message":"aborting due to 4 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 4 previous errors\n\n"}
[00:46:45] {"message":"Some errors occurred: E0277, E0308.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0277, E0308.\n"}
[00:46:45] {"message":"For more information about an error, try `rustc --explain E0277`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about a/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:46:45] 
[00:46:45] 
[00:46:45] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:46:45] Build completed unsuccessfully in 0:03:03
[00:46:45] Build completed unsuccessfully in 0:03:03
[00:46:45] make: *** [check] Error 1
[00:46:45] Makefile:58: recipe for target 'check' failed
128744 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release
127132 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu
127128 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
124364 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
