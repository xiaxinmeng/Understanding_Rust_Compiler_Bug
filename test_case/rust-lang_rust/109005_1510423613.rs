plain

94 LL | #[std::test]
95    |        ^^^^ could not find `test` in `std`
96    |
- note: found an item that was configured out
+ note: crate `std` has an item named `test` but it is inactive because its cfg predicate evaluated to false
98   --> $SRC_DIR/std/src/lib.rs:LL:COL
100 error: aborting due to 16 previous errors


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/builtin-std-paths-fail/builtin-std-paths-fail.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args macros/builtin-std-paths-fail.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/macros/builtin-std-paths-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/builtin-std-paths-fail" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/builtin-std-paths-fail/auxiliary"
stdout: none
--- stderr -------------------------------
  --> fake-test-src-base/macros/builtin-std-paths-fail.rs:2:11
   |
   |
LL |     core::RustcDecodable, //~ ERROR could not find `RustcDecodable` in `core`

error[E0433]: failed to resolve: could not find `RustcDecodable` in `core`
  --> fake-test-src-base/macros/builtin-std-paths-fail.rs:4:11
   |
   |
LL |     core::RustcDecodable, //~ ERROR could not find `RustcDecodable` in `core`

error[E0433]: failed to resolve: could not find `RustcDecodable` in `core`
  --> fake-test-src-base/macros/builtin-std-paths-fail.rs:2:11
   |
   |
LL |     core::RustcDecodable, //~ ERROR could not find `RustcDecodable` in `core`

error[E0433]: failed to resolve: could not find `RustcDecodable` in `core`
  --> fake-test-src-base/macros/builtin-std-paths-fail.rs:4:11
   |
   |
LL |     core::RustcDecodable, //~ ERROR could not find `RustcDecodable` in `core`


error[E0433]: failed to resolve: could not find `bench` in `core`
  --> fake-test-src-base/macros/builtin-std-paths-fail.rs:7:9
   |
LL | #[core::bench] //~ ERROR could not find `bench` in `core`
   |         ^^^^^ could not find `bench` in `core`
error[E0433]: failed to resolve: could not find `global_allocator` in `core`
  --> fake-test-src-base/macros/builtin-std-paths-fail.rs:8:9
   |
   |
LL | #[core::global_allocator] //~ ERROR could not find `global_allocator` in `core`
   |         ^^^^^^^^^^^^^^^^ could not find `global_allocator` in `core`
error[E0433]: failed to resolve: could not find `test_case` in `core`
  --> fake-test-src-base/macros/builtin-std-paths-fail.rs:9:9
   |
   |
LL | #[core::test_case] //~ ERROR could not find `test_case` in `core`
   |         ^^^^^^^^^ could not find `test_case` in `core`
error[E0433]: failed to resolve: could not find `test` in `core`
  --> fake-test-src-base/macros/builtin-std-paths-fail.rs:10:9
   |
   |
LL | #[core::test] //~ ERROR could not find `test` in `core`
   |         ^^^^ could not find `test` in `core`
error[E0433]: failed to resolve: could not find `RustcDecodable` in `std`
  --> fake-test-src-base/macros/builtin-std-paths-fail.rs:14:10
   |
   |
LL |     std::RustcDecodable, //~ ERROR could not find `RustcDecodable` in `std`

error[E0433]: failed to resolve: could not find `RustcDecodable` in `std`
  --> fake-test-src-base/macros/builtin-std-paths-fail.rs:16:10
   |
   |
LL |     std::RustcDecodable, //~ ERROR could not find `RustcDecodable` in `std`

error[E0433]: failed to resolve: could not find `RustcDecodable` in `std`
  --> fake-test-src-base/macros/builtin-std-paths-fail.rs:14:10
   |
   |
LL |     std::RustcDecodable, //~ ERROR could not find `RustcDecodable` in `std`

error[E0433]: failed to resolve: could not find `RustcDecodable` in `std`
  --> fake-test-src-base/macros/builtin-std-paths-fail.rs:16:10
   |
   |
LL |     std::RustcDecodable, //~ ERROR could not find `RustcDecodable` in `std`


error[E0433]: failed to resolve: could not find `bench` in `std`
  --> fake-test-src-base/macros/builtin-std-paths-fail.rs:19:8
   |
LL | #[std::bench] //~ ERROR could not find `bench` in `std`
   |        ^^^^^ could not find `bench` in `std`
error[E0433]: failed to resolve: could not find `global_allocator` in `std`
  --> fake-test-src-base/macros/builtin-std-paths-fail.rs:20:8
   |
   |
LL | #[std::global_allocator] //~ ERROR could not find `global_allocator` in `std`
   |        ^^^^^^^^^^^^^^^^ could not find `global_allocator` in `std`
error[E0433]: failed to resolve: could not find `test_case` in `std`
  --> fake-test-src-base/macros/builtin-std-paths-fail.rs:21:8
   |
   |
LL | #[std::test_case] //~ ERROR could not find `test_case` in `std`
   |        ^^^^^^^^^ could not find `test_case` in `std`
error[E0433]: failed to resolve: could not find `test` in `std`
  --> fake-test-src-base/macros/builtin-std-paths-fail.rs:22:8
   |
   |
LL | #[std::test] //~ ERROR could not find `test` in `std`
   |        ^^^^ could not find `test` in `std`
   |
note: crate `std` has an item named `test` but it is inactive because its cfg predicate evaluated to false
  --> /rustc/FAKE_PREFIX/library/std/src/lib.rs:379:1
error: aborting due to 16 previous errors

For more information about this error, try `rustc --explain E0433`.
------------------------------------------
------------------------------------------


---- [ui] tests/ui/macros/macro-outer-attributes.rs stdout ----
diff of stderr:

4 LL |     a::bar();
5    |        ^^^ not found in `a`
- note: found an item that was configured out
- note: found an item that was configured out
+ note: crate `macro_outer_attributes` has an item named `bar` but it is inactive because its cfg predicate evaluated to false
9    |
9    |
10 LL |                        $i:item) => (mod $nm { #[$a] $i }); }

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-outer-attributes/macro-outer-attributes.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args macros/macro-outer-attributes.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/macros/macro-outer-attributes.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-outer-attributes" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-outer-attributes/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0425]: cannot find function `bar` in module `a`
  --> fake-test-src-base/macros/macro-outer-attributes.rs:18:8
   |
LL |     a::bar(); //~ ERROR cannot find function `bar` in module `a`
   |        ^^^ not found in `a`
   |
note: crate `macro_outer_attributes` has an item named `bar` but it is inactive because its cfg predicate evaluated to false
  --> fake-test-src-base/macros/macro-outer-attributes.rs:5:45
   |
LL |                        $i:item) => (mod $nm { #[$a] $i }); }
LL |
LL |
LL | / test!(a,
LL | |       #[cfg(qux)],
LL | |       pub fn bar() { });
   | |______________^^^______- in this macro invocation
help: consider importing this function
   |
   |
LL + use b::bar;
help: if you import `bar`, refer to it directly
   |
   |
LL -     a::bar(); //~ ERROR cannot find function `bar` in module `a`
LL +     bar(); //~ ERROR cannot find function `bar` in module `a`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0425`.
