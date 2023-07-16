plain

---- [ui] src/test/ui/closures/2229_closure_analysis/migrations/auto_traits.rs stdout ----
diff of stderr:

29 LL |     thread::spawn(move || unsafe {
31    |                   |
31    |                   |
-    |                   in Rust 2018, this closure implements `Send` as `fptr` implements `Send`, but in Rust 2021, this closure will no longer implement `Send` because `fptr` is not fully captured and `fptr.0.0` does not implement `Send`
33    |                   in Rust 2018, this closure implements `Sync` as `fptr` implements `Sync`, but in Rust 2021, this closure will no longer implement `Sync` because `fptr` is not fully captured and `fptr.0.0` does not implement `Sync`
+    |                   in Rust 2018, this closure implements `Send` as `fptr` implements `Send`, but in Rust 2021, this closure will no longer implement `Send` because `fptr` is not fully captured and `fptr.0.0` does not implement `Send`
34 ...
35 LL |         *fptr.0.0 = 20;
36    |         --------- in Rust 2018, this closure captures all of `fptr`, but in Rust 2021, it will only capture `fptr.0.0`
Some tests failed in compiletest suite=ui mode=ui host=i686-unknown-linux-gnu target=i686-unknown-linux-gnu

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/i686-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/migrations/auto_traits/auto_traits.stderr
Actual stderr saved to /checkout/obj/build/i686-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/migrations/auto_traits/auto_traits.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args closures/2229_closure_analysis/migrations/auto_traits.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/i686-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closures/2229_closure_analysis/migrations/auto_traits.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/i686-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/migrations/auto_traits" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/i686-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/migrations/auto_traits/auxiliary"
stdout: none
--- stderr -------------------------------
error: changes to closure capture in Rust 2021 will affect which traits the closure implements
   |
   |
LL |     thread::spawn(move || unsafe {
   |                   ^^^^^^^^^^^^^^ in Rust 2018, this closure implements `Send` as `fptr` implements `Send`, but in Rust 2021, this closure will no longer implement `Send` because `fptr` is not fully captured and `fptr.0` does not implement `Send`
...
LL |         *fptr.0 = 20;
   |         ------- in Rust 2018, this closure captures all of `fptr`, but in Rust 2021, it will only capture `fptr.0`
note: the lint level is defined here
  --> /checkout/src/test/ui/closures/2229_closure_analysis/migrations/auto_traits.rs:2:9
   |
LL | #![deny(rust_2021_incompatible_closure_captures)]
LL | #![deny(rust_2021_incompatible_closure_captures)]
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/disjoint-capture-in-closures.html>
help: add a dummy let to cause `fptr` to be fully captured
   |
LL ~     thread::spawn(move || { let _ = &fptr; unsafe {
LL |         //~^ ERROR: changes to closure capture
LL |         //~| NOTE: in Rust 2018, this closure implements `Send`
LL |         //~| NOTE: for more information, see
LL |         //~| HELP: add a dummy let to cause `fptr` to be fully captured
LL |         *fptr.0 = 20;


error: changes to closure capture in Rust 2021 will affect which traits the closure implements
   |
   |
LL |     thread::spawn(move || unsafe {
   |                   |
   |                   |
   |                   in Rust 2018, this closure implements `Sync` as `fptr` implements `Sync`, but in Rust 2021, this closure will no longer implement `Sync` because `fptr` is not fully captured and `fptr.0.0` does not implement `Sync`
   |                   in Rust 2018, this closure implements `Send` as `fptr` implements `Send`, but in Rust 2021, this closure will no longer implement `Send` because `fptr` is not fully captured and `fptr.0.0` does not implement `Send`
...
LL |         *fptr.0.0 = 20;
   |         --------- in Rust 2018, this closure captures all of `fptr`, but in Rust 2021, it will only capture `fptr.0.0`
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/disjoint-capture-in-closures.html>
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/disjoint-capture-in-closures.html>
help: add a dummy let to cause `fptr` to be fully captured
   |
LL ~     thread::spawn(move || { let _ = &fptr; unsafe {
LL |         //~^ ERROR: changes to closure capture
LL |         //~| NOTE: in Rust 2018, this closure implements `Sync`
LL |         //~| NOTE: in Rust 2018, this closure implements `Send`
LL |         //~| NOTE: for more information, see
LL |         //~| HELP: add a dummy let to cause `fptr` to be fully captured


error: changes to closure capture in Rust 2021 will affect drop order and which traits the closure implements
   |
LL |     let c = || {
LL |     let c = || {
   |             ^^ in Rust 2018, this closure implements `Clone` as `f` implements `Clone`, but in Rust 2021, this closure will no longer implement `Clone` because `f` is not fully captured and `f.1` does not implement `Clone`
...
LL |         let f_1 = f.1;
   |                   --- in Rust 2018, this closure captures all of `f`, but in Rust 2021, it will only capture `f.1`
LL | }
LL | }
   | - in Rust 2018, `f` is dropped here, but in Rust 2021, only `f.1` will be dropped here as part of the closure
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/disjoint-capture-in-closures.html>
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/disjoint-capture-in-closures.html>
help: add a dummy let to cause `f` to be fully captured
   |
LL ~     let c = || {
LL +         let _ = &f;

error: aborting due to 3 previous errors
------------------------------------------



---- [ui] src/test/ui/closures/2229_closure_analysis/migrations/mir_calls_to_shims.rs stdout ----
diff of stderr:

4 LL |     let result = panic::catch_unwind(move || {
5    |                                      ^^^^^^^
6    |                                      |
-    |                                      in Rust 2018, this closure implements `RefUnwindSafe` as `f` implements `RefUnwindSafe`, but in Rust 2021, this closure will no longer implement `RefUnwindSafe` because `f` is not fully captured and `f.0` does not implement `RefUnwindSafe`
8    |                                      in Rust 2018, this closure implements `UnwindSafe` as `f` implements `UnwindSafe`, but in Rust 2021, this closure will no longer implement `UnwindSafe` because `f` is not fully captured and `f.0` does not implement `UnwindSafe`
+    |                                      in Rust 2018, this closure implements `RefUnwindSafe` as `f` implements `RefUnwindSafe`, but in Rust 2021, this closure will no longer implement `RefUnwindSafe` because `f` is not fully captured and `f.0` does not implement `RefUnwindSafe`
10 LL |         f.0()
10 LL |         f.0()
11    |         --- in Rust 2018, this closure captures all of `f`, but in Rust 2021, it will only capture `f.0`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/i686-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/migrations/mir_calls_to_shims/mir_calls_to_shims.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args closures/2229_closure_analysis/migrations/mir_calls_to_shims.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/i686-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closures/2229_closure_analysis/migrations/mir_calls_to_shims.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/i686-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/migrations/mir_calls_to_shims" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/i686-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/migrations/mir_calls_to_shims/auxiliary"
stdout: none
--- stderr -------------------------------
error: changes to closure capture in Rust 2021 will affect which traits the closure implements
   |
LL |     let result = panic::catch_unwind(move || {
   |                                      ^^^^^^^
   |                                      |
   |                                      |
   |                                      in Rust 2018, this closure implements `UnwindSafe` as `f` implements `UnwindSafe`, but in Rust 2021, this closure will no longer implement `UnwindSafe` because `f` is not fully captured and `f.0` does not implement `UnwindSafe`
   |                                      in Rust 2018, this closure implements `RefUnwindSafe` as `f` implements `RefUnwindSafe`, but in Rust 2021, this closure will no longer implement `RefUnwindSafe` because `f` is not fully captured and `f.0` does not implement `RefUnwindSafe`
LL |         f.0()
LL |         f.0()
   |         --- in Rust 2018, this closure captures all of `f`, but in Rust 2021, it will only capture `f.0`
note: the lint level is defined here
  --> /checkout/src/test/ui/closures/2229_closure_analysis/migrations/mir_calls_to_shims.rs:4:9
   |
LL | #![deny(rust_2021_incompatible_closure_captures)]
LL | #![deny(rust_2021_incompatible_closure_captures)]
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/disjoint-capture-in-closures.html>
help: add a dummy let to cause `f` to be fully captured
   |
LL ~     let result = panic::catch_unwind(move || {
LL +         let _ = &f;

error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/closures/2229_closure_analysis/migrations/multi_diagnostics.rs stdout ----
diff of stderr:

94 LL |     thread::spawn(move || unsafe {
96    |                   |
96    |                   |
-    |                   in Rust 2018, this closure implements `Send` as `fptr1` implements `Send`, but in Rust 2021, this closure will no longer implement `Send` because `fptr1` is not fully captured and `fptr1.0.0` does not implement `Send`
98    |                   in Rust 2018, this closure implements `Sync` as `fptr1` implements `Sync`, but in Rust 2021, this closure will no longer implement `Sync` because `fptr1` is not fully captured and `fptr1.0.0` does not implement `Sync`
+    |                   in Rust 2018, this closure implements `Send` as `fptr1` implements `Send`, but in Rust 2021, this closure will no longer implement `Send` because `fptr1` is not fully captured and `fptr1.0.0` does not implement `Send`
99    |                   in Rust 2018, this closure implements `Send` as `fptr2` implements `Send`, but in Rust 2021, this closure will no longer implement `Send` because `fptr2` is not fully captured and `fptr2.0` does not implement `Send`
100 ...
101 LL |         *fptr1.0.0 = 20;

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/i686-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/migrations/multi_diagnostics/multi_diagnostics.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args closures/2229_closure_analysis/migrations/multi_diagnostics.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/i686-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closures/2229_closure_analysis/migrations/multi_diagnostics.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/i686-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/migrations/multi_diagnostics" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/i686-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/migrations/multi_diagnostics/auxiliary"
stdout: none
--- stderr -------------------------------
error: changes to closure capture in Rust 2021 will affect drop order and which traits the closure implements
   |
LL |     let c = || {
LL |     let c = || {
   |             ^^ in Rust 2018, this closure implements `Clone` as `f1` implements `Clone`, but in Rust 2021, this closure will no longer implement `Clone` because `f1` is not fully captured and `f1.0` does not implement `Clone`
...
LL |         let _f_1 = f1.0;
   |                    ---- in Rust 2018, this closure captures all of `f1`, but in Rust 2021, it will only capture `f1.0`
LL |         //~^ NOTE: in Rust 2018, this closure captures all of `f1`, but in Rust 2021, it will only capture `f1.0`
LL |         let _f_2 = f2.1;
   |                    ---- in Rust 2018, this closure captures all of `f2`, but in Rust 2021, it will only capture `f2.1`
LL | }
LL | }
   | - in Rust 2018, `f2` is dropped here, but in Rust 2021, only `f2.1` will be dropped here as part of the closure
note: the lint level is defined here
  --> /checkout/src/test/ui/closures/2229_closure_analysis/migrations/multi_diagnostics.rs:2:9
   |
LL | #![deny(rust_2021_incompatible_closure_captures)]
LL | #![deny(rust_2021_incompatible_closure_captures)]
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/disjoint-capture-in-closures.html>
help: add a dummy let to cause `f1`, `f2` to be fully captured
   |
LL ~     let c = || {
LL +         let _ = (&f1, &f2);


error: changes to closure capture in Rust 2021 will affect which traits the closure implements
   |
LL |     let c = || {
LL |     let c = || {
   |             ^^ in Rust 2018, this closure implements `Clone` as `f1` implements `Clone`, but in Rust 2021, this closure will no longer implement `Clone` because `f1` is not fully captured and `f1.0` does not implement `Clone`
...
LL |         let _f_1 = f1.0;
   |                    ---- in Rust 2018, this closure captures all of `f1`, but in Rust 2021, it will only capture `f1.0`
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/disjoint-capture-in-closures.html>
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/disjoint-capture-in-closures.html>
help: add a dummy let to cause `f1` to be fully captured
   |
LL ~     let c = || {
LL +         let _ = &f1;


error: changes to closure capture in Rust 2021 will affect which traits the closure implements
   |
LL |     let c = || {
   |             ^^
   |             |
   |             |
   |             in Rust 2018, this closure implements `Clone` as `f1` implements `Clone`, but in Rust 2021, this closure will no longer implement `Clone` because `f1` is not fully captured and `f1.0` does not implement `Clone`
   |             in Rust 2018, this closure implements `Clone` as `f1` implements `Clone`, but in Rust 2021, this closure will no longer implement `Clone` because `f1` is not fully captured and `f1.2` does not implement `Clone`
...
LL |         let _f_0 = f1.0;
   |                    ---- in Rust 2018, this closure captures all of `f1`, but in Rust 2021, it will only capture `f1.0`
LL |         //~^ NOTE: in Rust 2018, this closure captures all of `f1`, but in Rust 2021, it will only capture `f1.0`
LL |         let _f_2 = f1.2;
   |                    ---- in Rust 2018, this closure captures all of `f1`, but in Rust 2021, it will only capture `f1.2`
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/disjoint-capture-in-closures.html>
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/disjoint-capture-in-closures.html>
help: add a dummy let to cause `f1` to be fully captured
   |
LL ~     let c = || {
LL +         let _ = &f1;


error: changes to closure capture in Rust 2021 will affect drop order and which traits the closure implements
   |
LL |     let c = || {
LL |     let c = || {
   |             ^^ in Rust 2018, this closure implements `Clone` as `f1` implements `Clone`, but in Rust 2021, this closure will no longer implement `Clone` because `f1` is not fully captured and `f1.0` does not implement `Clone`
...
LL |         let _f_0 = f1.0;
   |                    ---- in Rust 2018, this closure captures all of `f1`, but in Rust 2021, it will only capture `f1.0`
LL |         //~^ NOTE: in Rust 2018, this closure captures all of `f1`, but in Rust 2021, it will only capture `f1.0`
LL |         let _f_1 = f1.1;
   |                    ---- in Rust 2018, this closure captures all of `f1`, but in Rust 2021, it will only capture `f1.1`
LL | }
   | -
   | |
   | |
   | in Rust 2018, `f1` is dropped here, but in Rust 2021, only `f1.0` will be dropped here as part of the closure
   | in Rust 2018, `f1` is dropped here, but in Rust 2021, only `f1.1` will be dropped here as part of the closure
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/disjoint-capture-in-closures.html>
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/disjoint-capture-in-closures.html>
help: add a dummy let to cause `f1` to be fully captured
   |
LL ~     let c = || {
LL +         let _ = &f1;


error: changes to closure capture in Rust 2021 will affect which traits the closure implements
   |
   |
LL |     thread::spawn(move || unsafe {
   |                   |
   |                   |
   |                   in Rust 2018, this closure implements `Sync` as `fptr1` implements `Sync`, but in Rust 2021, this closure will no longer implement `Sync` because `fptr1` is not fully captured and `fptr1.0.0` does not implement `Sync`
   |                   in Rust 2018, this closure implements `Send` as `fptr1` implements `Send`, but in Rust 2021, this closure will no longer implement `Send` because `fptr1` is not fully captured and `fptr1.0.0` does not implement `Send`
   |                   in Rust 2018, this closure implements `Send` as `fptr2` implements `Send`, but in Rust 2021, this closure will no longer implement `Send` because `fptr2` is not fully captured and `fptr2.0` does not implement `Send`
...
LL |         *fptr1.0.0 = 20;
   |         ---------- in Rust 2018, this closure captures all of `fptr1`, but in Rust 2021, it will only capture `fptr1.0.0`
LL |         //~^ NOTE: in Rust 2018, this closure captures all of `fptr1`, but in Rust 2021, it will only capture `fptr1.0.0`
LL |         *fptr2.0 = 20;
   |         -------- in Rust 2018, this closure captures all of `fptr2`, but in Rust 2021, it will only capture `fptr2.0`
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/disjoint-capture-in-closures.html>
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/disjoint-capture-in-closures.html>
help: add a dummy let to cause `fptr1`, `fptr2` to be fully captured
   |
LL ~     thread::spawn(move || { let _ = (&fptr1, &fptr2); unsafe {
LL |         //~^ ERROR: changes to closure capture in Rust 2021
LL |         //~| NOTE: in Rust 2018, this closure implements `Sync` as `fptr1` implements `Sync`
LL |         //~| NOTE: in Rust 2018, this closure implements `Send` as `fptr1` implements `Send`
LL |         //~| NOTE: in Rust 2018, this closure implements `Send` as `fptr2` implements `Send`
LL |         //~| NOTE: for more information, see

error: aborting due to 5 previous errors
------------------------------------------

