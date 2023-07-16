plain
    Finished release [optimized] target(s) in 21.80s
     Running unittests lib.rs (obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/rustdoc-a4b4cac55e8385b8)

running 89 tests
..............................F..FF..................................................... 88/89

failures:

---- doctest::tests::make_test_crate_name stdout ----
---- doctest::tests::make_test_crate_name stdout ----
thread 'doctest::tests::make_test_crate_name' panicked at 'assertion failed: `(left == right)`
  left: `("#![allow(unused)]\n#[allow(unused_extern_crates)]\nextern crate r#asdf;\nfn main() {\nuse asdf::qwop;\nassert_eq!(2+2, 4);\n}", 4)`,
 right: `("#![allow(unused)]\n#[allow(unused_extern_crates)]\nextern crate r#asdf;\nfn main() {\nuse asdf::qwop;\nassert_eq!(2+2, 4);\n}", 3)`', src/librustdoc/doctest/tests.rs:49:5

---- doctest::tests::make_test_opts_attrs stdout ----
thread 'doctest::tests::make_test_opts_attrs' panicked at 'assertion failed: `(left == right)`
thread 'doctest::tests::make_test_opts_attrs' panicked at 'assertion failed: `(left == right)`
  left: `("#![feature(sick_rad)]\n#[allow(unused_extern_crates)]\nextern crate r#asdf;\nfn main() {\nuse asdf::qwop;\nassert_eq!(2+2, 4);\n}", 4)`,
 right: `("#![feature(sick_rad)]\n#[allow(unused_extern_crates)]\nextern crate r#asdf;\nfn main() {\nuse asdf::qwop;\nassert_eq!(2+2, 4);\n}", 3)`', src/librustdoc/doctest/tests.rs:140:5
---- doctest::tests::make_test_issues_21299_33731 stdout ----
---- doctest::tests::make_test_issues_21299_33731 stdout ----
thread 'doctest::tests::make_test_issues_21299_33731' panicked at 'assertion failed: `(left == right)`
  left: `("#![allow(unused)]\nextern crate hella_qwop;\n#[allow(unused_extern_crates)]\nextern crate r#asdf;\nfn main() {\nassert_eq!(asdf::foo, 4);\n}", 4)`,
 right: `("#![allow(unused)]\nextern crate hella_qwop;\n#[allow(unused_extern_crates)]\nextern crate r#asdf;\nfn main() {\nassert_eq!(asdf::foo, 4);\n}", 3)`', src/librustdoc/doctest/tests.rs:250:5

failures:
    doctest::tests::make_test_crate_name
    doctest::tests::make_test_opts_attrs
    doctest::tests::make_test_opts_attrs
    doctest::tests::make_test_issues_21299_33731

test result: FAILED. 86 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.48ms

error: test failed, to rerun pass `-p rustdoc --lib`
