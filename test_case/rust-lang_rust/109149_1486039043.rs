plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:8f4b7f84864484a7bf31766abe9204da3cbe65b3)
Download action repository 'rust-lang/simpleinfra@master' (SHA:13c1b4e09b845ddb9664cee13d03879444a1054d)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-16core-64gb)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
....................

failures:

---- [ui] tests/ui/macros/missing-writer.rs stdout ----

31    |
31    |
32 LL |     write!("{}_{}", x, y);
- help: A writer is needed before this format string
+ help: a writer is needed before this format string
35   --> $DIR/missing-writer.rs:5:12
36    |
36    |
37 LL |     write!("{}_{}", x, y);
48    |
48    |
49 LL |     writeln!("{}_{}", x, y);
- help: A writer is needed before this format string
+ help: a writer is needed before this format string
52   --> $DIR/missing-writer.rs:11:14
53    |
53    |
54 LL |     writeln!("{}_{}", x, y);

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/missing-writer/missing-writer.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args macros/missing-writer.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/macros/missing-writer.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/missing-writer" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/missing-writer/auxiliary"
stdout: none
--- stderr -------------------------------
error: format argument must be a string literal
  --> fake-test-src-base/macros/missing-writer.rs:5:21
   |
LL |     write!("{}_{}", x, y);
   |
help: you might be missing a string literal to format with
   |
   |
LL |     write!("{}_{}", "{} {}", x, y);

error: format argument must be a string literal
  --> fake-test-src-base/macros/missing-writer.rs:11:23
   |
   |
LL |     writeln!("{}_{}", x, y);
   |
help: you might be missing a string literal to format with
   |
   |
LL |     writeln!("{}_{}", "{} {}", x, y);

error[E0599]: cannot write into `&'static str`
  --> fake-test-src-base/macros/missing-writer.rs:5:12
   |
   |
LL |     write!("{}_{}", x, y);
   |     -------^^^^^^^------- method not found in `&str`
   |
note: must implement `io::Write`, `fmt::Write`, or have a `write_fmt` method
  --> fake-test-src-base/macros/missing-writer.rs:5:12
   |
LL |     write!("{}_{}", x, y);
help: a writer is needed before this format string
  --> fake-test-src-base/macros/missing-writer.rs:5:12
   |
   |
LL |     write!("{}_{}", x, y);

error[E0599]: cannot write into `&'static str`
  --> fake-test-src-base/macros/missing-writer.rs:11:14
   |
   |
LL |     writeln!("{}_{}", x, y);
   |     ---------^^^^^^^------- method not found in `&str`
   |
note: must implement `io::Write`, `fmt::Write`, or have a `write_fmt` method
  --> fake-test-src-base/macros/missing-writer.rs:11:14
   |
LL |     writeln!("{}_{}", x, y);
help: a writer is needed before this format string
  --> fake-test-src-base/macros/missing-writer.rs:11:14
   |
   |
LL |     writeln!("{}_{}", x, y);

error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0599`.
