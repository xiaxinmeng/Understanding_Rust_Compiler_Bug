plain
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/in-trait/early-bound-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/in-trait/early-bound-1" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/in-trait/early-bound-1/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0670]: `async fn` is not permitted in Rust 2015
   |
LL |     async fn foo(&mut self);
LL |     async fn foo(&mut self);
   |     ^^^^^ to use `async fn`, switch to Rust 2018 or later
   |
   = help: pass `--edition 2021` to `rustc`
   = note: for more on editions, read https://doc.rust-lang.org/edition-guide

error[E0670]: `async fn` is not permitted in Rust 2015
   |
LL |     async fn foo(&mut self) {}
LL |     async fn foo(&mut self) {}
   |     ^^^^^ to use `async fn`, switch to Rust 2018 or later
   |
   = help: pass `--edition 2021` to `rustc`
   = note: for more on editions, read https://doc.rust-lang.org/edition-guide
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0670`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/async-await/in-trait/early-bound-2.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/in-trait/early-bound-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/in-trait/early-bound-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/in-trait/early-bound-2/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0670]: `async fn` is not permitted in Rust 2015
   |
LL |     async fn foo(&mut self);
LL |     async fn foo(&mut self);
   |     ^^^^^ to use `async fn`, switch to Rust 2018 or later
   |
   = help: pass `--edition 2021` to `rustc`
   = note: for more on editions, read https://doc.rust-lang.org/edition-guide

error[E0670]: `async fn` is not permitted in Rust 2015
   |
LL |     async fn foo(&mut self) {}
LL |     async fn foo(&mut self) {}
   |     ^^^^^ to use `async fn`, switch to Rust 2018 or later
   |
   = help: pass `--edition 2021` to `rustc`
   = note: for more on editions, read https://doc.rust-lang.org/edition-guide
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0670`.
------------------------------------------
