plain
---- [ui] tests/ui/thread-local/thread-local-static-ref-use-after-free.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/thread-local/thread-local-static-ref-use-after-free.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/thread-local/thread-local-static-ref-use-after-free" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/thread-local/thread-local-static-ref-use-after-free/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0670]: `async fn` is not permitted in Rust 2015
  --> fake-test-src-base/thread-local/thread-local-static-ref-use-after-free.rs:17:1
LL | async fn bar() {}
LL | async fn bar() {}
   | ^^^^^ to use `async fn`, switch to Rust 2018 or later
   |
   = help: pass `--edition 2021` to `rustc`
   = note: for more on editions, read https://doc.rust-lang.org/edition-guide

error[E0670]: `async fn` is not permitted in Rust 2015
  --> fake-test-src-base/thread-local/thread-local-static-ref-use-after-free.rs:20:1
LL | async fn foo() {
LL | async fn foo() {
   | ^^^^^ to use `async fn`, switch to Rust 2018 or later
   |
   = help: pass `--edition 2021` to `rustc`
   = note: for more on editions, read https://doc.rust-lang.org/edition-guide

error[E0609]: no field `await` on type `impl Future<Output = ()>`
  --> fake-test-src-base/thread-local/thread-local-static-ref-use-after-free.rs:22:11
LL |     bar().await;
LL |     bar().await;
   |           ^^^^^ field not found in `impl Future<Output = ()>`
   |
   = note: to `.await` a `Future`, switch to Rust 2018 or later
   = help: pass `--edition 2021` to `rustc`
   = note: for more on editions, read https://doc.rust-lang.org/edition-guide
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0609, E0670.
For more information about an error, try `rustc --explain E0609`.
