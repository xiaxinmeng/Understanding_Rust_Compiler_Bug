plain
      System Firmware Version: VMW71.00V.13989454.B64.1906190538
      OS Loader Version: 540.120.3~22
      Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
      SMC Version (system): 2.8f0
      Serial Number (system): VM0JroWfjSS6
      Provisioning UDID: 4203018E-580F-C1B5-9525-B745CECA79EB

hw.ncpu: 12
hw.byteorder: 1234
---
test [pretty] tests/pretty/trait-inner-attr.rs ... ok
test [pretty] tests/pretty/stmt_expr_attributes.rs ... ok
test [pretty] tests/pretty/struct-tuple.rs ... ok
test [pretty] tests/pretty/trait-polarity.rs ... ok
2023-03-10T15:14:06.665790Z ERROR compiletest::runtest: fatal error, panic: "pretty-printed source does not match expected source\nexpected:\n------------------------------------------\n#![feature(prelude_import)]\n#![no_std]\n#[prelude_import]\nuse ::std::prelude::rust_2015::*;\n#[macro_use]\nextern crate std;\n// compile-flags: --crate-type=lib --test\n// pretty-compare-only\n// pretty-mode:expanded\n// pp-exact:tests-are-sorted.pp\n\nextern crate test;\n#[cfg(test)]\n#[rustc_test_marker = \"m_test\"]\npub const m_test: test::TestDescAndFn =\n    test::TestDescAndFn {\n        desc: test::TestDesc {\n            name: test::StaticTestName(\"m_test\"),\n            ignore: false,\n            ignore_message: ::core::option::Option::None,\n            source_file: \"/checkout/tests/pretty/tests-are-sorted.rs\",\n            start_line: 7usize,\n            start_col: 4usize,\n            end_line: 7usize,\n            end_col: 10usize,\n            compile_fail: false,\n            no_run: false,\n            should_panic: test::ShouldPanic::No,\n            test_type: test::TestType::Unknown,\n        },\n        testfn: test::StaticTestFn(|| test::assert_test_result(m_test())),\n    };\nfn m_test() {}\n\nextern crate test;\n#[cfg(test)]\n#[rustc_test_marker = \"z_test\"]\npub const z_test: test::TestDescAndFn =\n    test::TestDescAndFn {\n        desc: test::TestDesc {\n            name: test::StaticTestName(\"z_test\"),\n            ignore: true,\n            ignore_message: ::core::option::Option::Some(\"not yet implemented\"),\n            source_file: \"/checkout/tests/pretty/tests-are-sorted.rs\",\n            start_line: 11usize,\n            start_col: 4usize,\n            end_line: 11usize,\n            end_col: 10usize,\n            compile_fail: false,\n            no_run: false,\n            should_panic: test::ShouldPanic::No,\n            test_type: test::TestType::Unknown,\n        },\n        testfn: test::StaticTestFn(|| test::assert_test_result(z_test())),\n    };\n#[ignore = \"not yet implemented\"]\nfn z_test() {}\n\nextern crate test;\n#[cfg(test)]\n#[rustc_test_marker = \"a_test\"]\npub const a_test: test::TestDescAndFn =\n    test::TestDescAndFn {\n        desc: test::TestDesc {\n            name: test::StaticTestName(\"a_test\"),\n            ignore: false,\n            ignore_message: ::core::option::Option::None,\n            source_file: \"/checkout/tests/pretty/tests-are-sorted.rs\",\n            start_line: 14usize,\n            start_col: 4usize,\n            end_line: 14usize,\n            end_col: 10usize,\n            compile_fail: false,\n            no_run: false,\n            should_panic: test::ShouldPanic::No,\n            test_type: test::TestType::Unknown,\n        },\n        testfn: test::StaticTestFn(|| test::assert_test_result(a_test())),\n    };\nfn a_test() {}\n#[rustc_main]\npub fn main() -> () {\n    extern crate test;\n    test::test_main_static(&[&a_test, &m_test, &z_test])\n}\n\n------------------------------------------\nactual:\n------------------------------------------\n#![feature(prelude_import)]\n#![no_std]\n#[prelude_import]\nuse ::std::prelude::rust_2015::*;\n#[macro_use]\nextern crate std;\n// compile-flags: --crate-type=lib --test\n// pretty-compare-only\n// pretty-mode:expanded\n// pp-exact:tests-are-sorted.pp\n\nextern crate test;\n#[cfg(test)]\n#[rustc_test_marker = \"m_test\"]\npub const m_test: test::TestDescAndFn =\n    test::TestDescAndFn {\n        desc: test::TestDesc {\n            name: test::StaticTestName(\"m_test\"),\n            ignore: false,\n            ignore_message: ::core::option::Option::None,\n            source_file: \"/Users/runner/work/rust/rust/tests/pretty/tests-are-sorted.rs\",\n            start_line: 7usize,\n            start_col: 4usize,\n            end_line: 7usize,\n            end_col: 10usize,\n            compile_fail: false,\n            no_run: false,\n            should_panic: test::ShouldPanic::No,\n            test_type: test::TestType::Unknown,\n        },\n        testfn: test::StaticTestFn(|| test::assert_test_result(m_test())),\n    };\nfn m_test() {}\n\nextern crate test;\n#[cfg(test)]\n#[rustc_test_marker = \"z_test\"]\npub const z_test: test::TestDescAndFn =\n    test::TestDescAndFn {\n        desc: test::TestDesc {\n            name: test::StaticTestName(\"z_test\"),\n            ignore: true,\n            ignore_message: ::core::option::Option::Some(\"not yet implemented\"),\n            source_file: \"/Users/runner/work/rust/rust/tests/pretty/tests-are-sorted.rs\",\n            start_line: 11usize,\n            start_col: 4usize,\n            end_line: 11usize,\n            end_col: 10usize,\n            compile_fail: false,\n            no_run: false,\n            should_panic: test::ShouldPanic::No,\n            test_type: test::TestType::Unknown,\n        },\n        testfn: test::StaticTestFn(|| test::assert_test_result(z_test())),\n    };\n#[ignore = \"not yet implemented\"]\nfn z_test() {}\n\nextern crate test;\n#[cfg(test)]\n#[rustc_test_marker = \"a_test\"]\npub const a_test: test::TestDescAndFn =\n    test::TestDescAndFn {\n        desc: test::TestDesc {\n            name: test::StaticTestName(\"a_test\"),\n            ignore: false,\n            ignore_message: ::core::option::Option::None,\n            source_file: \"/Users/runner/work/rust/rust/tests/pretty/tests-are-sorted.rs\",\n            start_line: 14usize,\n            start_col: 4usize,\n            end_line: 14usize,\n            end_col: 10usize,\n            compile_fail: false,\n            no_run: false,\n            should_panic: test::ShouldPanic::No,\n            test_type: test::TestType::Unknown,\n        },\n        testfn: test::StaticTestFn(|| test::assert_test_result(a_test())),\n    };\nfn a_test() {}\n#[rustc_main]\npub fn main() -> () {\n    extern crate test;\n    test::test_main_static(&[&a_test, &m_test, &z_test])\n}\n\n------------------------------------------\ndiff:\n------------------------------------------\n18\t            name: test::StaticTestName(\"m_test\"),\n19\t            ignore: false,\n20\t            ignore_message: ::core::option::Option::None,\n-\t            source_file: \"/checkout/tests/pretty/tests-are-sorted.rs\",\n+\t            source_file: \"/Users/runner/work/rust/rust/tests/pretty/tests-are-sorted.rs\",\n22\t            start_line: 7usize,\n23\t            start_col: 4usize,\n24\t            end_line: 7usize,\n\n41\t            name: test::StaticTestName(\"z_test\"),\n42\t            ignore: true,\n43\t            ignore_message: ::core::option::Option::Some(\"not yet implemented\"),\n-\t            source_file: \"/checkout/tests/pretty/tests-are-sorted.rs\",\n+\t            source_file: \"/Users/runner/work/rust/rust/tests/pretty/tests-are-sorted.rs\",\n45\t            start_line: 11usize,\n46\t            start_col: 4usize,\n47\t            end_line: 11usize,\n\n65\t            name: test::StaticTestName(\"a_test\"),\n66\t            ignore: false,\n67\t            ignore_message: ::core::option::Option::None,\n-\t            source_file: \"/checkout/tests/pretty/tests-are-sorted.rs\",\n+\t            source_file: \"/Users/runner/work/rust/rust/tests/pretty/tests-are-sorted.rs\",\n69\t            start_line: 14usize,\n70\t            start_col: 4usize,\n71\t            end_line: 14usize,\n\n\n"
test [pretty] tests/pretty/use-tree.rs ... ok
test [pretty] tests/pretty/trait-safety.rs ... ok
test [pretty] tests/pretty/attr-derive.rs ... ok
test [pretty] tests/pretty/vec-comments.rs ... ok
---

error: pretty-printed source does not match expected source
expected:
------------------------------------------
#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use ::std::prelude::rust_2015::*;
extern crate std;
// compile-flags: --crate-type=lib --test
// pretty-compare-only
// pretty-mode:expanded
// pretty-mode:expanded
// pp-exact:tests-are-sorted.pp
extern crate test;
#[cfg(test)]
#[cfg(test)]
#[rustc_test_marker = "m_test"]
pub const m_test: test::TestDescAndFn =
    test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("m_test"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            start_line: 7usize,
            start_col: 4usize,
            end_line: 7usize,
            end_col: 10usize,
            end_col: 10usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::Unknown,
        },
        testfn: test::StaticTestFn(|| test::assert_test_result(m_test())),
fn m_test() {}

extern crate test;
#[cfg(test)]
#[cfg(test)]
#[rustc_test_marker = "z_test"]
pub const z_test: test::TestDescAndFn =
    test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("z_test"),
            ignore: true,
            ignore_message: ::core::option::Option::Some("not yet implemented"),
            start_line: 11usize,
            start_col: 4usize,
            end_line: 11usize,
            end_col: 10usize,
            end_col: 10usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::Unknown,
        },
        testfn: test::StaticTestFn(|| test::assert_test_result(z_test())),
#[ignore = "not yet implemented"]
fn z_test() {}

extern crate test;
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "a_test"]
pub const a_test: test::TestDescAndFn =
    test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("a_test"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            start_line: 14usize,
            start_col: 4usize,
            end_line: 14usize,
            end_col: 10usize,
            end_col: 10usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::Unknown,
        },
        testfn: test::StaticTestFn(|| test::assert_test_result(a_test())),
fn a_test() {}
#[rustc_main]
pub fn main() -> () {
    extern crate test;
    extern crate test;
    test::test_main_static(&[&a_test, &m_test, &z_test])

------------------------------------------
actual:
------------------------------------------
------------------------------------------
#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use ::std::prelude::rust_2015::*;
extern crate std;
// compile-flags: --crate-type=lib --test
// pretty-compare-only
// pretty-mode:expanded
// pretty-mode:expanded
// pp-exact:tests-are-sorted.pp
extern crate test;
#[cfg(test)]
#[cfg(test)]
#[rustc_test_marker = "m_test"]
pub const m_test: test::TestDescAndFn =
    test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("m_test"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            start_line: 7usize,
            start_col: 4usize,
            end_line: 7usize,
            end_col: 10usize,
            end_col: 10usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::Unknown,
        },
        testfn: test::StaticTestFn(|| test::assert_test_result(m_test())),
fn m_test() {}

extern crate test;
#[cfg(test)]
#[cfg(test)]
#[rustc_test_marker = "z_test"]
pub const z_test: test::TestDescAndFn =
    test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("z_test"),
            ignore: true,
            ignore_message: ::core::option::Option::Some("not yet implemented"),
            start_line: 11usize,
            start_col: 4usize,
            end_line: 11usize,
            end_col: 10usize,
            end_col: 10usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::Unknown,
        },
        testfn: test::StaticTestFn(|| test::assert_test_result(z_test())),
#[ignore = "not yet implemented"]
fn z_test() {}

extern crate test;
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "a_test"]
pub const a_test: test::TestDescAndFn =
    test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("a_test"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            start_line: 14usize,
            start_col: 4usize,
            end_line: 14usize,
            end_col: 10usize,
            end_col: 10usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::Unknown,
        },
        testfn: test::StaticTestFn(|| test::assert_test_result(a_test())),
fn a_test() {}
#[rustc_main]
pub fn main() -> () {
    extern crate test;
    extern crate test;
    test::test_main_static(&[&a_test, &m_test, &z_test])

------------------------------------------
diff:
------------------------------------------
------------------------------------------
18             name: test::StaticTestName("m_test"),
19             ignore: false,
20             ignore_message: ::core::option::Option::None,
+             source_file: "/Users/runner/work/rust/rust/tests/pretty/tests-are-sorted.rs",
22             start_line: 7usize,
23             start_col: 4usize,
24             end_line: 7usize,
24             end_line: 7usize,

41             name: test::StaticTestName("z_test"),
42             ignore: true,
43             ignore_message: ::core::option::Option::Some("not yet implemented"),
+             source_file: "/Users/runner/work/rust/rust/tests/pretty/tests-are-sorted.rs",
45             start_line: 11usize,
46             start_col: 4usize,
47             end_line: 11usize,
47             end_line: 11usize,

65             name: test::StaticTestName("a_test"),
66             ignore: false,
67             ignore_message: ::core::option::Option::None,
+             source_file: "/Users/runner/work/rust/rust/tests/pretty/tests-are-sorted.rs",
69             start_line: 14usize,
70             start_col: 4usize,
71             end_line: 14usize,
