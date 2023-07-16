plain
........iii............................................................................. 13112/13139
...........................
failures:

---- [ui] src/test/ui/inference/need_type_info/channel.rs stdout ----

1 error[E0282]: type annotations needed
-   --> $DIR/channel.rs:5:9
+   --> $DIR/channel.rs:8:9
---
To only update this specific test, also pass `--test-args inference/need_type_info/channel.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/inference/need_type_info/channel.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/need_type_info/channel" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/need_type_info/channel/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/inference/need_type_info/channel.rs:8:9
   |
   |
LL |         channel(); //~ ERROR type annotations needed
   |         ^^^^^^^ cannot infer type of the type parameter `T` declared on the function `channel`
help: consider specifying the generic argument
   |
   |
LL |         channel::<T>(); //~ ERROR type annotations needed

error[E0282]: type annotations needed
  --> /checkout/src/test/ui/inference/need_type_info/channel.rs:13:9
   |
   |
LL |         channel(); //~ ERROR type annotations needed
   |         ^^^^^^^ cannot infer type of the type parameter `T` declared on the function `channel`
help: consider specifying the generic argument
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |
   |
LL |         channel::<T>(); //~ ERROR type annotations needed

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0282`.
---

1 error[E0282]: type annotations needed
2   --> $DIR/issue-25368.rs:11:27
3    |
- LL |         tx.send(Foo{ foo: PhantomData }); 
+ LL |         tx.send(Foo{ foo: PhantomData });
5    |                           ^^^^^^^^^^^ cannot infer type of the type parameter `T` declared on the struct `PhantomData`
7 help: consider specifying the generic argument

8    |
8    |
- LL |         tx.send(Foo{ foo: PhantomData::<T> }); 
+ LL |         tx.send(Foo{ foo: PhantomData::<T> });
11 
12 error: aborting due to previous error



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-25368/issue-25368.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-25368.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-25368.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-25368" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-25368/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/issues/issue-25368.rs:11:27
   |
   |
LL |         tx.send(Foo{ foo: PhantomData });
   |                           ^^^^^^^^^^^ cannot infer type of the type parameter `T` declared on the struct `PhantomData`
help: consider specifying the generic argument
   |
   |
LL |         tx.send(Foo{ foo: PhantomData::<T> });

error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.
