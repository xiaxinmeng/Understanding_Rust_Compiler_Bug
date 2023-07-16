plain
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii

Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
 finished in 0.072 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i...ii...i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.28s

 finished in 2.357 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
    Finished release [optimized] target(s) in 25.01s
     Running build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/rustdoc-c6465bc84f83bbc5

running 51 tests
...........F....F.....F............................

---- doctest::tests::make_test_crate_name stdout ----
---- doctest::tests::make_test_crate_name stdout ----
thread 'doctest::tests::make_test_crate_name' panicked at 'assertion failed: `(left == right)`
  left: `("#![allow(unused)]\nextern crate r#asdf;\nfn main() {\nuse asdf::qwop;\nassert_eq!(2+2, 4);\n}", 3)`,
 right: `("#![allow(unused)]\nextern crate asdf;\nfn main() {\nuse asdf::qwop;\nassert_eq!(2+2, 4);\n}", 3)`', src/librustdoc/doctest/tests.rs:48:5

---- doctest::tests::make_test_issues_21299_33731 stdout ----
---- doctest::tests::make_test_issues_21299_33731 stdout ----
thread 'doctest::tests::make_test_issues_21299_33731' panicked at 'assertion failed: `(left == right)`
  left: `("#![allow(unused)]\nextern crate hella_qwop;\nextern crate r#asdf;\nfn main() {\nassert_eq!(asdf::foo, 4);\n}", 3)`,
 right: `("#![allow(unused)]\nextern crate hella_qwop;\nextern crate asdf;\nfn main() {\nassert_eq!(asdf::foo, 4);\n}", 3)`', src/librustdoc/doctest/tests.rs:260:5
---- doctest::tests::make_test_opts_attrs stdout ----
---- doctest::tests::make_test_opts_attrs stdout ----
thread 'doctest::tests::make_test_opts_attrs' panicked at 'assertion failed: `(left == right)`
  left: `("#![feature(sick_rad)]\nextern crate r#asdf;\nfn main() {\nuse asdf::qwop;\nassert_eq!(2+2, 4);\n}", 3)`,
 right: `("#![feature(sick_rad)]\nextern crate asdf;\nfn main() {\nuse asdf::qwop;\nassert_eq!(2+2, 4);\n}", 3)`', src/librustdoc/doctest/tests.rs:138:5

failures:
    doctest::tests::make_test_crate_name
    doctest::tests::make_test_issues_21299_33731
    doctest::tests::make_test_issues_21299_33731
    doctest::tests::make_test_opts_attrs

test result: FAILED. 48 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

error: test failed, to rerun pass '-p rustdoc --lib'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rustdoc/Cargo.toml" "-p" "rustdoc:0.0.0" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:26:33
