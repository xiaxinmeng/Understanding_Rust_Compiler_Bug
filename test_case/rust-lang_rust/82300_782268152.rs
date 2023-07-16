plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
    Checking rand_core v0.5.1
error[E0061]: this function takes 7 arguments but 6 arguments were supplied
   --> library/test/src/tests.rs:97:5
    |
97  |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess, tx, Concurrent::No);
    |     |
    |     expected 7 arguments
    |
note: function defined here
note: function defined here
   --> library/test/src/lib.rs:452:8
    |
452 | pub fn run_test(
    |        ^^^^^^^^
453 |     opts: &TestOpts,
454 |     force_ignore: bool,
    |     ------------------
455 |     id: TestId,
    |     ----------
    |     ----------
456 |     test: TestDescAndFn,
    |     -------------------
457 |     strategy: RunStrategy,
    |     ---------------------
458 |     monitor_ch: Sender<CompletedTest>,
459 |     concurrency: Concurrent,
    |     -----------------------

error[E0061]: this function takes 7 arguments but 6 arguments were supplied
error[E0061]: this function takes 7 arguments but 6 arguments were supplied
   --> library/test/src/tests.rs:116:5
    |
116 |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess, tx, Concurrent::No);
    |     |
    |     expected 7 arguments
    |
note: function defined here
note: function defined here
   --> library/test/src/lib.rs:452:8
    |
452 | pub fn run_test(
    |        ^^^^^^^^
453 |     opts: &TestOpts,
454 |     force_ignore: bool,
    |     ------------------
455 |     id: TestId,
    |     ----------
    |     ----------
456 |     test: TestDescAndFn,
    |     -------------------
457 |     strategy: RunStrategy,
    |     ---------------------
458 |     monitor_ch: Sender<CompletedTest>,
459 |     concurrency: Concurrent,
    |     -----------------------

error[E0061]: this function takes 7 arguments but 6 arguments were supplied
error[E0061]: this function takes 7 arguments but 6 arguments were supplied
   --> library/test/src/tests.rs:139:5
    |
139 |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess, tx, Concurrent::No);
    |     |
    |     expected 7 arguments
    |
note: function defined here
note: function defined here
   --> library/test/src/lib.rs:452:8
    |
452 | pub fn run_test(
    |        ^^^^^^^^
453 |     opts: &TestOpts,
454 |     force_ignore: bool,
    |     ------------------
455 |     id: TestId,
    |     ----------
    |     ----------
456 |     test: TestDescAndFn,
    |     -------------------
457 |     strategy: RunStrategy,
    |     ---------------------
458 |     monitor_ch: Sender<CompletedTest>,
459 |     concurrency: Concurrent,
    |     -----------------------

error[E0061]: this function takes 7 arguments but 6 arguments were supplied
error[E0061]: this function takes 7 arguments but 6 arguments were supplied
   --> library/test/src/tests.rs:162:5
    |
162 |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess, tx, Concurrent::No);
    |     |
    |     expected 7 arguments
    |
note: function defined here
note: function defined here
   --> library/test/src/lib.rs:452:8
    |
452 | pub fn run_test(
    |        ^^^^^^^^
453 |     opts: &TestOpts,
454 |     force_ignore: bool,
    |     ------------------
455 |     id: TestId,
    |     ----------
    |     ----------
456 |     test: TestDescAndFn,
    |     -------------------
457 |     strategy: RunStrategy,
    |     ---------------------
458 |     monitor_ch: Sender<CompletedTest>,
459 |     concurrency: Concurrent,
    |     -----------------------

error[E0061]: this function takes 7 arguments but 6 arguments were supplied
error[E0061]: this function takes 7 arguments but 6 arguments were supplied
   --> library/test/src/tests.rs:190:5
    |
190 |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess, tx, Concurrent::No);
    |     |
    |     expected 7 arguments
    |
note: function defined here
note: function defined here
   --> library/test/src/lib.rs:452:8
    |
452 | pub fn run_test(
    |        ^^^^^^^^
453 |     opts: &TestOpts,
454 |     force_ignore: bool,
    |     ------------------
455 |     id: TestId,
    |     ----------
    |     ----------
456 |     test: TestDescAndFn,
    |     -------------------
457 |     strategy: RunStrategy,
    |     ---------------------
458 |     monitor_ch: Sender<CompletedTest>,
459 |     concurrency: Concurrent,
    |     -----------------------

error[E0061]: this function takes 7 arguments but 6 arguments were supplied
error[E0061]: this function takes 7 arguments but 6 arguments were supplied
   --> library/test/src/tests.rs:222:5
    |
222 |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess, tx, Concurrent::No);
    |     |
    |     expected 7 arguments
    |
note: function defined here
note: function defined here
   --> library/test/src/lib.rs:452:8
    |
452 | pub fn run_test(
    |        ^^^^^^^^
453 |     opts: &TestOpts,
454 |     force_ignore: bool,
    |     ------------------
455 |     id: TestId,
    |     ----------
    |     ----------
456 |     test: TestDescAndFn,
    |     -------------------
457 |     strategy: RunStrategy,
    |     ---------------------
458 |     monitor_ch: Sender<CompletedTest>,
459 |     concurrency: Concurrent,
    |     -----------------------

error[E0061]: this function takes 7 arguments but 6 arguments were supplied
error[E0061]: this function takes 7 arguments but 6 arguments were supplied
   --> library/test/src/tests.rs:246:9
    |
246 |         run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess, tx, Concurrent::No);
    |         |
    |         expected 7 arguments
    |
note: function defined here
note: function defined here
   --> library/test/src/lib.rs:452:8
    |
452 | pub fn run_test(
    |        ^^^^^^^^
453 |     opts: &TestOpts,
454 |     force_ignore: bool,
    |     ------------------
455 |     id: TestId,
    |     ----------
    |     ----------
456 |     test: TestDescAndFn,
    |     -------------------
457 |     strategy: RunStrategy,
    |     ---------------------
458 |     monitor_ch: Sender<CompletedTest>,
459 |     concurrency: Concurrent,
    |     -----------------------

error[E0061]: this function takes 7 arguments but 6 arguments were supplied
error[E0061]: this function takes 7 arguments but 6 arguments were supplied
   --> library/test/src/tests.rs:273:5
    |
273 |     run_test(&test_opts, false, desc, RunStrategy::InProcess, tx, Concurrent::No);
    |     |
    |     expected 7 arguments
    |
note: function defined here
note: function defined here
   --> library/test/src/lib.rs:452:8
    |
452 | pub fn run_test(
    |        ^^^^^^^^
453 |     opts: &TestOpts,
454 |     force_ignore: bool,
    |     ------------------
455 |     id: TestId,
    |     ----------
    |     ----------
456 |     test: TestDescAndFn,
    |     -------------------
457 |     strategy: RunStrategy,
    |     ---------------------
458 |     monitor_ch: Sender<CompletedTest>,
459 |     concurrency: Concurrent,
    |     -----------------------

error[E0061]: this function takes 7 arguments but 6 arguments were supplied
error[E0061]: this function takes 7 arguments but 6 arguments were supplied
   --> library/test/src/tests.rs:308:5
    |
308 |     run_test(&test_opts, false, desc, RunStrategy::InProcess, tx, Concurrent::No);
    |     |
    |     expected 7 arguments
    |
note: function defined here
note: function defined here
   --> library/test/src/lib.rs:452:8
    |
452 | pub fn run_test(
    |        ^^^^^^^^
453 |     opts: &TestOpts,
454 |     force_ignore: bool,
    |     ------------------
455 |     id: TestId,
    |     ----------
    |     ----------
456 |     test: TestDescAndFn,
    |     -------------------
457 |     strategy: RunStrategy,
    |     ---------------------
458 |     monitor_ch: Sender<CompletedTest>,
459 |     concurrency: Concurrent,
    |     -----------------------

error[E0061]: this function takes 5 arguments but 4 arguments were supplied
error[E0061]: this function takes 5 arguments but 4 arguments were supplied
   --> library/test/src/tests.rs:640:5
    |
640 |     crate::bench::benchmark(desc, tx, true, f);
    |     ^^^^^^^^^^^^^^^^^^^^^^^ ----  --  ----  - supplied 4 arguments
    |     expected 5 arguments
    |
note: function defined here
   --> library/test/src/bench.rs:184:8
   --> library/test/src/bench.rs:184:8
    |
184 | pub fn benchmark<F>(
185 |     id: TestId,
    |     ----------
186 |     desc: TestDesc,
    |     --------------
    |     --------------
187 |     monitor_ch: Sender<CompletedTest>,
188 |     nocapture: bool,
    |     ---------------
189 |     f: F,
    |     ----
    |     ----

error[E0061]: this function takes 5 arguments but 4 arguments were supplied
   --> library/test/src/tests.rs:660:5
    |
660 |     crate::bench::benchmark(desc, tx, true, f);
    |     ^^^^^^^^^^^^^^^^^^^^^^^ ----  --  ----  - supplied 4 arguments
    |     expected 5 arguments
    |
note: function defined here
   --> library/test/src/bench.rs:184:8
   --> library/test/src/bench.rs:184:8
    |
184 | pub fn benchmark<F>(
185 |     id: TestId,
    |     ----------
186 |     desc: TestDesc,
    |     --------------
    |     --------------
187 |     monitor_ch: Sender<CompletedTest>,
188 |     nocapture: bool,
    |     ---------------
189 |     f: F,
    |     ----
---

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--all-targets" "-p" "test" "-p" "core" "-p" "panic_unwind" "-p" "unwind" "-p" "alloc" "-p" "std" "-p" "proc_macro" "-p" "panic_abort" "-p" "term" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:00:36
