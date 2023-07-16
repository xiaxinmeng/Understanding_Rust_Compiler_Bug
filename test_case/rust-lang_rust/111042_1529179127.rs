plain
 finished in 41.575 seconds
Check compiletest suite=pretty mode=pretty (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 73 tests
......................................................2023-05-01T00:12:58.713007Z ERROR compiletest::runtest: fatal error, panic: "pretty-printed source does not match expected source\nexpected:\n------------------------------------------\n#![feature(prelude_import)]\n#![no_std]\n#[prelude_import]\nuse ::std::prelude::rust_2015::*;\n#[macro_use]\nextern crate std;\n// compile-flags: --crate-type=lib --test --remap-path-prefix={{src-base}}/=/the/src/ --remap-path-prefix={{src-base}}\\=/the/src/\n// pretty-compare-only\n// pretty-mode:expanded\n// pp-exact:tests-are-sorted.pp\n\nextern crate test;\n#[cfg(test)]\n#[rustc_test_marker = \"m_test\"]\npub const m_test: test::TestDescAndFn =\n    test::TestDescAndFn {\n        desc: test::TestDesc {\n            name: test::StaticTestName(\"m_test\"),\n            ignore: false,\n            ignore_message: ::core::option::Option::None,\n            source_file: \"/the/src/tests-are-sorted.rs\",\n            start_line: 7usize,\n            start_col: 4usize,\n            end_line: 7usize,\n            end_col: 10usize,\n            compile_fail: false,\n            no_run: false,\n            should_panic: test::ShouldPanic::No,\n            test_type: test::TestType::Unknown,\n        },\n        testfn: test::StaticTestFn(|| test::assert_test_result(m_test())),\n    };\nfn m_test() {}\n\nextern crate test;\n#[cfg(test)]\n#[rustc_test_marker = \"z_test\"]\npub const z_test: test::TestDescAndFn =\n    test::TestDescAndFn {\n        desc: test::TestDesc {\n            name: test::StaticTestName(\"z_test\"),\n            ignore: true,\n            ignore_message: ::core::option::Option::Some(\"not yet implemented\"),\n            source_file: \"/the/src/tests-are-sorted.rs\",\n            start_line: 11usize,\n            start_col: 4usize,\n            end_line: 11usize,\n            end_col: 10usize,\n            compile_fail: false,\n            no_run: false,\n            should_panic: test::ShouldPanic::No,\n            test_type: test::TestType::Unknown,\n        },\n        testfn: test::StaticTestFn(|| test::assert_test_result(z_test())),\n    };\n#[ignore = \"not yet implemented\"]\nfn z_test() {}\n\nextern crate test;\n#[cfg(test)]\n#[rustc_test_marker = \"a_test\"]\npub const a_test: test::TestDescAndFn =\n    test::TestDescAndFn {\n        desc: test::TestDesc {\n            name: test::StaticTestName(\"a_test\"),\n            ignore: false,\n            ignore_message: ::core::option::Option::None,\n            source_file: \"/the/src/tests-are-sorted.rs\",\n            start_line: 14usize,\n            start_col: 4usize,\n            end_line: 14usize,\n            end_col: 10usize,\n            compile_fail: false,\n            no_run: false,\n            should_panic: test::ShouldPanic::No,\n            test_type: test::TestType::Unknown,\n        },\n        testfn: test::StaticTestFn(|| test::assert_test_result(a_test())),\n    };\nfn a_test() {}\n#[rustc_main]\npub fn main() -> () {\n    extern crate test;\n    test::test_main_static(&[&a_test, &m_test, &z_test])\n}\n\n------------------------------------------\nactual:\n------------------------------------------\n#![feature(prelude_import)]\n#![no_std]\n#[prelude_import]\nuse ::std::prelude::rust_2015::*;\n#[macro_use]\nextern crate std;\n// compile-flags: --crate-type=lib --test --remap-path-prefix={{src-base}}/=/the/src/ --remap-path-prefix={{src-base}}\\=/the/src/\n// pretty-compare-only\n// pretty-mode:expanded\n// pp-exact:tests-are-sorted.pp\n\nextern crate test;\n#[cfg(test)]\n#[rustc_test_marker = \"m_test\"]\npub const m_test: test::TestDescAndFn =\n    test::TestDescAndFn {\n        desc: test::TestDesc {\n            name: test::StaticTestName(\"m_test\"),\n            ignore: false,\n            ignore_message: ::core::option::Option::None,\n            source_file: \"/the/src/tests-are-sorted.rs\",\n            start_line: 7usize,\n            start_col: 4usize,\n            end_line: 7usize,\n            end_col: 10usize,\n            compile_fail: false,\n            no_run: false,\n            should_panic: test::ShouldPanic::No,\n            test_type: test::TestType::Unknown,\n        },\n        testfn: test::StaticTestFn(|| test::assert_test_result(m_test())),\n    };\nfn m_test() {}\n\nextern crate test;\n#[cfg(test)]\n#[rustc_test_marker = \"z_test\"]\npub const z_test: test::TestDescAndFn =\n    test::TestDescAndFn {\n        desc: test::TestDesc {\n            name: test::StaticTestName(\"z_test\"),\n            ignore: true,\n            ignore_message: ::core::option::Option::Some(\"not yet implemented\"),\n            source_file: \"/the/src/tests-are-sorted.rs\",\n            start_line: 11usize,\n            start_col: 4usize,\n            end_line: 11usize,\n            end_col: 10usize,\n            compile_fail: false,\n            no_run: false,\n            should_panic: test::ShouldPanic::No,\n            test_type: test::TestType::Unknown,\n        },\n        testfn: test::StaticTestFn(|| test::assert_test_result(z_test())),\n    };\n#[ignore = \"not yet implemented\"]\nfn z_test() {}\n\nextern crate test;\n#[cfg(test)]\n#[rustc_test_marker = \"a_test\"]\npub const a_test: test::TestDescAndFn =\n    test::TestDescAndFn {\n        desc: test::TestDesc {\n            name: test::StaticTestName(\"a_test\"),\n            ignore: false,\n            ignore_message: ::core::option::Option::None,\n            source_file: \"/the/src/tests-are-sorted.rs\",\n            start_line: 14usize,\n            start_col: 4usize,\n            end_line: 14usize,\n            end_col: 10usize,\n            compile_fail: false,\n            no_run: false,\n            should_panic: test::ShouldPanic::No,\n            test_type: test::TestType::Unknown,\n        },\n        testfn: test::StaticTestFn(|| test::assert_test_result(a_test())),\n    };\nfn a_test() {}\n#[rustc_main]\n#[no_coverage]\npub fn main() -> () {\n    extern crate test;\n    test::test_main_static(&[&a_test, &m_test, &z_test])\n}\n\n------------------------------------------\ndiff:\n------------------------------------------\n79\t    };\n80\tfn a_test() {}\n81\t#[rustc_main]\n+\t#[no_coverage]\n82\tpub fn main() -> () {\n83\t    extern crate test;\n84\t    test::test_main_static(&[&a_test, &m_test, &z_test])\n\n\n"
F..................
failures:

---- [pretty] tests/pretty/tests-are-sorted.rs stdout ----


error: pretty-printed source does not match expected source
expected:
------------------------------------------
#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use ::std::prelude::rust_2015::*;
extern crate std;
extern crate std;
// compile-flags: --crate-type=lib --test --remap-path-prefix={{src-base}}/=/the/src/ --remap-path-prefix={{src-base}}\=/the/src/
// pretty-mode:expanded
// pretty-mode:expanded
// pp-exact:tests-are-sorted.pp
extern crate test;
#[cfg(test)]
Build completed unsuccessfully in 0:13:32
Build completed unsuccessfully in 0:13:32
#[rustc_test_marker = "m_test"]
pub const m_test: test::TestDescAndFn =
    test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("m_test"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "/the/src/tests-are-sorted.rs",
            start_line: 7usize,
            start_col: 4usize,
            end_line: 7usize,
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
            source_file: "/the/src/tests-are-sorted.rs",
            start_line: 11usize,
            start_col: 4usize,
            end_line: 11usize,
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
            source_file: "/the/src/tests-are-sorted.rs",
            start_line: 14usize,
            start_col: 4usize,
            end_line: 14usize,
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
extern crate std;
// compile-flags: --crate-type=lib --test --remap-path-prefix={{src-base}}/=/the/src/ --remap-path-prefix={{src-base}}\=/the/src/
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
            source_file: "/the/src/tests-are-sorted.rs",
            start_line: 7usize,
            start_col: 4usize,
            end_line: 7usize,
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
            source_file: "/the/src/tests-are-sorted.rs",
            start_line: 11usize,
            start_col: 4usize,
            end_line: 11usize,
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
            source_file: "/the/src/tests-are-sorted.rs",
            start_line: 14usize,
            start_col: 4usize,
            end_line: 14usize,
            end_col: 10usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::Unknown,
        },
        testfn: test::StaticTestFn(|| test::assert_test_result(a_test())),
fn a_test() {}
#[rustc_main]
#[no_coverage]
pub fn main() -> () {
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&a_test, &m_test, &z_test])

------------------------------------------
diff:
------------------------------------------
------------------------------------------
79     };
80 fn a_test() {}
81 #[rustc_main]
+ #[no_coverage]
83     extern crate test;
83     extern crate test;
84     test::test_main_static(&[&a_test, &m_test, &z_test])


thread '[pretty] tests/pretty/tests-are-sorted.rs' panicked at 'fatal error', src/tools/compiletest/src/runtest.rs:2304:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
