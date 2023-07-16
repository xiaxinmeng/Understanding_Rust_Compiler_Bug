plain
Resolving deltas: 100% (611330/611330), completed with 4859 local objects.
---
[00:00:46] configure: rust.quiet-tests     := True
---
[00:40:30] ..........................................................................i.........................
[00:40:36] .................i.........................................................F........................
---
[00:41:11] .............................................................................................i......
[00:41:18] .................................................................i..................................
---
[00:41:40] - error[E0119]: conflicting implementations of trait `std::ops::Deref` for type `&_`:
[00:41:40] + error[E0119]: conflicting implementations of trait `std::ops::Deref` for type `std::cell::Ref<'_, _>`:
[00:41:40] 2   --> $DIR/issue-28981.rs:15:1
[00:41:40] 3    |
[00:41:40] 4 LL | impl<Foo> Deref for Foo { } //~ ERROR must be used
[00:41:40]
[00:41:40] 5    | ^^^^^^^^^^^^^^^^^^^^^^^
[00:41:40] 6    |
[00:41:40] 7    = note: conflicting implementation in crate `core`:
[00:41:40] -            - impl<'a, T> std::ops::Deref for &'a T
[00:41:40] +            - impl<'b, T> std::ops::Deref for std::cell::Ref<'b, T>
[00:41:40] 9              where T: ?Sized;
[00:41:40] 10
[00:41:40] 11 error[E0210]: type parameter `Foo` must be used as the type parameter for some local type (e.g. `MyStruct<Foo>`)
[00:41:40]
[00:41:40]
[00:41:40] The actual stderr differed from the expected stderr.
[00:41:40] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/e0119/issue-28981.stderr
[00:41:40] To update references, run this command from build directory:
[00:41:40] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'e0119/issue-28981.rs'
[00:41:40]
[00:41:40] error: 1 errors occurred comparing output.
[00:41:40] status: exit code: 101
[00:41:40] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/e0119/issue-28981.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/e0119/issue-28981.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/e0119/issue-28981.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
---
[00:41:40] {"message":"conflicting implementations of trait `std::ops::Deref` for type `std::cell::Ref<'_, _>`:","code":{"code":"E0119","explanation":"\nThere are conflicting trait implementations for the same type.\nExample of erroneous code:\n\n