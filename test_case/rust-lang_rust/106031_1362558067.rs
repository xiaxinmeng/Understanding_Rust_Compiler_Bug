plain
test [ui] src/test/ui/transmutability/primitives/numbers.rs ... ok

failures:

---- [ui] src/test/ui/generator/unresolved-ct-var.rs#stock stdout ----
thread '[ui] src/test/ui/generator/unresolved-ct-var.rs#stock' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 39, kind: DirectoryNotEmpty, message: "Directory not empty" }', src/tools/compiletest/src/runtest.rs:2871:47

---- [ui] src/test/ui/generator/unresolved-ct-var.rs#drop_tracking stdout ----
diff of stderr:


+ error: Failed to delete invalidated or incompatible incremental compilation session directory contents `$TEST_BUILD_DIR/generator/unresolved-ct-var/unresolved-ct-var.inc/unresolved_ct_var-25ai3o8jofo30/s-ggkfizoe8g-137zrl-working/dep-graph.bin`: No such file or directory (os error 2).
+ 
+ error: failed to create dependency graph at `$TEST_BUILD_DIR/generator/unresolved-ct-var/unresolved-ct-var.inc/unresolved_ct_var-25ai3o8jofo30/s-ggkfizoe8g-137zrl-working/dep-graph.part.bin`: No such file or directory (os error 2)
+ 
1 error[E0277]: `[(); _]` is not a future
3    |


72 LL |         let s = std::array::from_fn(|_| ()).await;
74 
- error: aborting due to 6 previous errors
+ error: aborting due to 8 previous errors
76 
76 
77 Some errors have detailed explanations: E0277, E0698.
78 For more information about an error, try `rustc --explain E0277`.


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/unresolved-ct-var.drop_tracking/unresolved-ct-var.drop_tracking.stderr
To only update this specific test, also pass `--test-args generator/unresolved-ct-var.rs`


error in revision `drop_tracking`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generator/unresolved-ct-var.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--cfg" "drop_tracking" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/unresolved-ct-var/unresolved-ct-var.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/unresolved-ct-var.drop_tracking" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=i586-unknown-linux-gnu-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/unresolved-ct-var.drop_tracking/auxiliary" "--edition=2021" "-Zdrop-tracking"
stdout: none
--- stderr -------------------------------
error: Failed to delete invalidated or incompatible incremental compilation session directory contents `/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/unresolved-ct-var/unresolved-ct-var.inc/unresolved_ct_var-25ai3o8jofo30/s-ggkfizoe8g-137zrl-working/dep-graph.bin`: No such file or directory (os error 2).

error: failed to create dependency graph at `/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/unresolved-ct-var/unresolved-ct-var.inc/unresolved_ct_var-25ai3o8jofo30/s-ggkfizoe8g-137zrl-working/dep-graph.part.bin`: No such file or directory (os error 2)

error[E0277]: `[(); _]` is not a future
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=i586-unknown-linux-gnu
   |
   |
LL |         let s = std::array::from_fn(|_| ()).await;
   |                 |                          |
   |                 |                          |
   |                 |                          `[(); _]` is not a future
   |                 |                          help: remove the `.await`
   |                 this call returns `[(); _]`
   |
   = help: the trait `Future` is not implemented for `[(); _]`
   = note: [(); _] must be a future or must implement `IntoFuture` to be awaited
   = note: required for `[(); _]` to implement `IntoFuture`

error[E0698]: type inside `async` block must be known in this context
   |
   |
LL |         let s = std::array::from_fn(|_| ()).await;
   |                 ^^^^^^^^^^^^^^^^^^^ cannot infer the value of const parameter `N` declared on the function `from_fn`
   |
note: the type is part of the `async` block because of this `await`
   |
   |
LL |         let s = std::array::from_fn(|_| ()).await;


error[E0698]: type inside `async` block must be known in this context
   |
   |
LL |         let s = std::array::from_fn(|_| ()).await;
   |                 ^^^^^^^^^^^^^^^^^^^ cannot infer the value of const parameter `N` declared on the function `from_fn`
   |
note: the type is part of the `async` block because of this `await`
   |
   |
LL |         let s = std::array::from_fn(|_| ()).await;


error[E0698]: type inside `async` block must be known in this context
   |
   |
LL |         let s = std::array::from_fn(|_| ()).await;
   |                 ^^^^^^^^^^^^^^^^^^^ cannot infer the value of const parameter `N` declared on the function `from_fn`
   |
note: the type is part of the `async` block because of this `await`
   |
   |
LL |         let s = std::array::from_fn(|_| ()).await;


error[E0698]: type inside `async` block must be known in this context
   |
   |
LL |         let s = std::array::from_fn(|_| ()).await;
   |                 ^^^^^^^^^^^^^^^^^^^ cannot infer the value of const parameter `N` declared on the function `from_fn`
   |
note: the type is part of the `async` block because of this `await`
   |
   |
LL |         let s = std::array::from_fn(|_| ()).await;


error[E0698]: type inside `async` block must be known in this context
   |
   |
LL |         let s = std::array::from_fn(|_| ()).await;
   |                 ^^^^^^^^^^^^^^^^^^^ cannot infer the value of const parameter `N` declared on the function `from_fn`
   |
note: the type is part of the `async` block because of this `await`
   |
   |
LL |         let s = std::array::from_fn(|_| ()).await;

error: aborting due to 8 previous errors

Some errors have detailed explanations: E0277, E0698.
