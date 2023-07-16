plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:24cb9080177205b6e8c946b17badbe402adc938f)
Download action repository 'rust-lang/simpleinfra@master' (SHA:5f3e9487b084c5235556ffd8baa8b183de9eb120)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-16core-64gb)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
diff of stderr:

21   --> $SRC_DIR/std/src/io/buffered/bufwriter.rs:LL:COL
22 
23 error[E0599]: the method `write_fmt` exists for struct `BufWriter<&dyn Write>`, but its trait bounds were not satisfied
-   --> $DIR/mut-borrow-needed-by-trait.rs:21:5
25    |
25    |
26 LL |     writeln!(fp, "hello world").unwrap();
-    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ method cannot be called on `BufWriter<&dyn Write>` due to unsatisfied trait bounds
+    |     ---------^^---------------- method cannot be called on `BufWriter<&dyn Write>` due to unsatisfied trait bounds
28   --> $SRC_DIR/std/src/io/buffered/bufwriter.rs:LL:COL
29    |
30    = note: doesn't satisfy `BufWriter<&dyn std::io::Write>: std::io::Write`
31    |
31    |
+ note: must implement 'io::Write', 'fmt::Write', or have a write_fmt method
+    |
+    |
+ LL |     writeln!(fp, "hello world").unwrap();
32    = note: the following trait bounds were not satisfied:
33            `&dyn std::io::Write: std::io::Write`
33            `&dyn std::io::Write: std::io::Write`
34            which is required by `BufWriter<&dyn std::io::Write>: std::io::Write`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
-    = note: this error originates in the macro `writeln` (in Nightly builds, run with -Z macro-backtrace for more info)
36 
37 error: aborting due to 3 previous errors
---
To only update this specific test, also pass `--test-args suggestions/mut-borrow-needed-by-trait.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/suggestions/mut-borrow-needed-by-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/mut-borrow-needed-by-trait" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/mut-borrow-needed-by-trait/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `&dyn std::io::Write: std::io::Write` is not satisfied
  --> fake-test-src-base/suggestions/mut-borrow-needed-by-trait.rs:17:29
   |
LL |     let fp = BufWriter::new(fp);
   |              -------------- ^^ the trait `std::io::Write` is not implemented for `&dyn std::io::Write`
   |              required by a bound introduced by this call
   |
   |
   = note: `std::io::Write` is implemented for `&mut dyn std::io::Write`, but not for `&dyn std::io::Write`
note: required by a bound in `BufWriter::<W>::new`
  --> /rustc/FAKE_PREFIX/library/std/src/io/buffered/bufwriter.rs:96:5
error[E0277]: the trait bound `&dyn std::io::Write: std::io::Write` is not satisfied
  --> fake-test-src-base/suggestions/mut-borrow-needed-by-trait.rs:17:14
   |
   |
LL |     let fp = BufWriter::new(fp);
   |              ^^^^^^^^^^^^^^^^^^ the trait `std::io::Write` is not implemented for `&dyn std::io::Write`
   |
   = note: `std::io::Write` is implemented for `&mut dyn std::io::Write`, but not for `&dyn std::io::Write`
note: required by a bound in `BufWriter`
  --> /rustc/FAKE_PREFIX/library/std/src/io/buffered/bufwriter.rs:70:1

error[E0599]: the method `write_fmt` exists for struct `BufWriter<&dyn Write>`, but its trait bounds were not satisfied
  --> fake-test-src-base/suggestions/mut-borrow-needed-by-trait.rs:21:14
   |
LL |     writeln!(fp, "hello world").unwrap(); //~ ERROR the method
   |     ---------^^---------------- method cannot be called on `BufWriter<&dyn Write>` due to unsatisfied trait bounds
  --> /rustc/FAKE_PREFIX/library/std/src/io/buffered/bufwriter.rs:70:1
   |
   = note: doesn't satisfy `BufWriter<&dyn std::io::Write>: std::io::Write`
   |
note: must implement 'io::Write', 'fmt::Write', or have a write_fmt method
  --> fake-test-src-base/suggestions/mut-borrow-needed-by-trait.rs:21:14
   |
LL |     writeln!(fp, "hello world").unwrap(); //~ ERROR the method
   = note: the following trait bounds were not satisfied:
           `&dyn std::io::Write: std::io::Write`
           `&dyn std::io::Write: std::io::Write`
           which is required by `BufWriter<&dyn std::io::Write>: std::io::Write`
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0277, E0599.
For more information about an error, try `rustc --explain E0277`.
