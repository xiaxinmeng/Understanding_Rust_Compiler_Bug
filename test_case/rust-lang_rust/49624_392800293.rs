plain
[00:53:21] 
[00:53:21] running 1471 tests
[00:53:25] .........................................................................................i..........
[00:53:31] ...............................................i....................................................
[00:53:36] .......F............................................................................................
[00:53:43] ....................................................................................................
[00:53:46] ....................................................................................................
[00:53:51] ....................................................................................................
[00:53:56] ....................................................................................................
[00:53:56] ....................................................................................................
[00:54:00] ....................................................................................................
[00:54:07] .......................................................................................i............
[00:54:12] ................................................................i...................................
[00:54:17] ....................................................................................................
[00:54:23] ....................................................................................................
[00:54:30] .............................................................................................i......
[00:54:33] ...........iiiiiiiii...................................................
[00:54:33] 
[00:54:33] ---- [ui] ui/e0119/issue-28981.rs stdout ----
[00:54:33] diff of stderr:
[00:54:33] 
[00:54:33] 
[00:54:33] - error[E0119]: conflicting implementations of trait `std::ops::Deref` for type `std::cell::Ref<'_, _>`:
[00:54:33] + error[E0119]: conflicting implementations of trait `std::ops::Deref` for type `std::mem::PinMut<'_, _>`:
[00:54:33] 2   --> $DIR/issue-28981.rs:15:1
[00:54:33] 3    |
[00:54:33] 4 LL | impl<Foo> Deref for Foo { } //~ ERROR must be used
[00:54:33] 5    | ^^^^^^^^^^^^^^^^^^^^^^^
[00:54:33] 6    |
[00:54:33] 7    = note: conflicting implementation in crate `core`:
[00:54:33] 7    = note: conflicting implementation in crate `core`:
[00:54:33] -            - impl<'b, T> std::ops::Deref for std::cell::Ref<'b, T>
[00:54:33] +            - impl<'a, T> std::ops::Deref for std::mem::PinMut<'a, T>
[00:54:33] 9              where T: ?Sized;
[00:54:33] 10 
[00:54:33] 11 error[E0210]: type parameter `Foo` must be used as the type parameter for some local type (e.g. `MyStruct<Foo>`)
[00:54:33] 
[00:54:33] The actual stderr differed from the expected stderr.
[00:54:33] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/e0119/issue-28981/issue-28981.stderr
[00:54:33] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/e0119/issue-28981/issue-28981.stderr
[00:54:33] To update references, rerun the tests and pass the `--bless` flag
[00:54:33] To only update this specific test, also pass `--test-args e0119/issue-28981.rs`
[00:54:33] error: 1 errors occurred comparing output.
[00:54:33] status: exit code: 101
[00:54:33] status: exit code: 101
[00:54:33] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/e0119/issue-28981.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/e0119/issue-28981/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/e0119/issue-28981/auxiliary" "-A" "unused"
[00:54:33] ------------------------------------------
[00:54:33] 
[00:54:33] ------------------------------------------
[00:54:33] stderr:
[00:54:33] stderr:
[00:54:33] ------------------------------------------
[00:54:33] {"message":"conflicting implementations of trait `std::ops::Deref` for type `std::mem::PinMut<'_, _>`:","code":{"code":"E0119","explanation":"\nThere are conflicting trait implementations for the same type.\nExample of erroneous code:\n\n