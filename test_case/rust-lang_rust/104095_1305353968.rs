plain
...............................iii...................................................... 13728/13781
.....................................................
failures:

---- [ui] src/test/ui/thread-available_parallelism.rs stdout ----
warning: unused variable: `n`
  --> $DIR/thread-available_parallelism.rs:10:9
   |
LL |     let n = std::thread::available_parallelism().expect(
---



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/thread-available_parallelism/thread-available_parallelism.stderr
To only update this specific test, also pass `--test-args thread-available_parallelism.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/thread-available_parallelism.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/thread-available_parallelism/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/thread-available_parallelism/auxiliary"
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
stdout: none
--- stderr -------------------------------
warning: unused variable: `n`
  --> /checkout/src/test/ui/thread-available_parallelism.rs:10:9
LL |     let n = std::thread::available_parallelism().expect(
   |         ^ help: if this is intentional, prefix it with an underscore: `_n`
   |
   = note: `#[warn(unused_variables)]` on by default
