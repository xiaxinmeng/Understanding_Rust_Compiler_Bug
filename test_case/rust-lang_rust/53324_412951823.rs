plain
[00:48:00] ...............................i....................................................................
[00:48:02] ....................................................................................................
[00:48:06] ....................................................................................................
[00:48:09] ....................................................................................................
[00:48:12] .....................F..............................................................................
[00:48:18] ....................................................................................................
[00:48:21] ....................................................................................................
[00:48:25] ....................................................................................................
[00:48:28] ......................i.............................................................................
---
[00:49:12] 
[00:49:12] ---- [ui] ui/issues/issue-36638.rs stdout ----
[00:49:12] diff of stderr:
[00:49:12] 
[00:49:12] 10 LL | trait Bar<Self> {}
[00:49:12] 12 
[00:49:12] - error: aborting due to 2 previous errors
[00:49:12] - error: aborting due to 2 previous errors
[00:49:12] + error[E0392]: parameter `Self` is never used
[00:49:12] +   --> $DIR/issue-36638.rs:13:12
[00:49:12] +    |
[00:49:12] + LL | struct Foo<Self>(Self);
[00:49:12] +    |            ^^^^ unused type parameter
[00:49:12] +    |
[00:49:12] +    = help: consider removing `Self` or using a marker such as `std::marker::PhantomData`
[00:49:12] + error: aborting due to 3 previous errors
[00:49:12] + 
[00:49:12] + For more information about this error, try `rustc --explain E0392`.
[00:49:12] 15 
[00:49:12] 15 
[00:49:12] 
[00:49:12] 
[00:49:12] The actual stderr differed from the expected stderr.
[00:49:12] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-36638/issue-36638.stderr
[00:49:12] To update references, rerun the tests and pass the `--bless` flag
[00:49:12] To only update this specific test, also pass `--test-args issues/issue-36638.rs`
[00:49:12] error: 1 errors occurred comparing output.
[00:49:12] error: 1 errors occurred comparing output.
[00:49:12] stat\nthat there are no actual uses of `'a`. It's possible to work around this\nby adding a PhantomData type to the struct, using it to tell the compiler\nto act as if the struct contained a borrowed reference `&'a T`:\n\n