rust
#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use ::std::prelude::v1::*;
#[macro_use]
extern crate std;
#[test]
pub fn panic() {

}
#[allow(unused)]
use self::panic as panic;
pub mod __test_reexports {
    pub use super::panic;
}
pub mod __test {
    extern crate test;
    #[main]
    pub fn main() -> () { test::test_main_static(TESTS) }
    const TESTS: &'static [self::test::TestDescAndFn] =
        &[self::test::TestDescAndFn{desc:
                                        self::test::TestDesc{name:
                                                                 self::test::StaticTestName("panic"),
                                                             ignore: false,
                                                             should_panic:
                                                                 self::test::ShouldPanic::No,
                                                             allow_fail:
                                                                 false,},
                                    testfn:
                                        self::test::StaticTestFn(||
                                                                     self::test::assert_test_result(::__test_reexports::panic())),}];
}
