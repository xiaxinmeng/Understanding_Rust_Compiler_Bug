plain
............................iii..................................................................... 12600/12633
.................................
failures:

---- [ui] ui/traits/pointee-tail-is-generic.rs stdout ----
error: test compilation failed although it shouldn't!
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/pointee-tail-is-generic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/pointee-tail-is-generic" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/pointee-tail-is-generic/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0422]: cannot find struct, variant or union type `async` in this scope
  --> /checkout/src/test/ui/traits/pointee-tail-is-generic.rs:9:5
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
LL |     async {}
LL |     async {}
   |     ^^^^^ `async` blocks are only allowed in Rust 2018 or later
error: aborting due to previous error

For more information about this error, try `rustc --explain E0422`.


------------------------------------------



failures:
    [ui] ui/traits/pointee-tail-is-generic.rs
test result: FAILED. 12504 passed; 1 failed; 128 ignored; 0 measured; 0 filtered out; finished in 95.50s

Build completed unsuccessfully in 0:10:15
