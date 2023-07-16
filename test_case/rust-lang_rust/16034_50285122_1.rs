 rust
#![feature(phase)]
#![no_std]
#![feature(globs)]
#[phase(plugin, link)]
extern crate std = "std";
extern crate rt = "native";
use std::prelude::*;
mod foo {
    use std::prelude::*;
    mod bar {
        use std::prelude::*;
        #[test]
        fn bar_test() { }
        pub mod __test_reexports {
            use std::prelude::*;
            pub use super::bar_test;
        }
    }
    #[test]
    fn foo_test() { }
    pub mod __test_reexports {
        use std::prelude::*;
        pub use super::foo_test;
        pub use bar = super::bar::__test_reexports;
    }
}
#[test]
fn global_test() { }
pub mod __test_reexports {
    use std::prelude::*;
    pub use super::global_test;
    pub use foo = super::foo::__test_reexports;
}
pub mod __test {
    extern crate test;
    use std::prelude::*;
    pub fn main() {
        #![main]
        use std::slice::Vector;
        test::test_main_static(::std::os::args().as_slice(), TESTS);
    }
    pub static TESTS: &'static [self::test::TestDescAndFn] =
        &[self::test::TestDescAndFn{desc:
                                        self::test::TestDesc{name:
                                                                 self::test::StaticTestName("foo::bar::bar_test"),
                                                             ignore: false,
                                                             should_fail:
                                                                 false,},
                                    testfn:
                                        self::test::StaticTestFn(::__test_reexports::foo::bar::bar_test),},
          self::test::TestDescAndFn{desc:
                                        self::test::TestDesc{name:
                                                                 self::test::StaticTestName("foo::foo_test"),
                                                             ignore: false,
                                                             should_fail:
                                                                 false,},
                                    testfn:
                                        self::test::StaticTestFn(::__test_reexports::foo::foo_test),},
          self::test::TestDescAndFn{desc:
                                        self::test::TestDesc{name:
                                                                 self::test::StaticTestName("global_test"),
                                                             ignore: false,
                                                             should_fail:
                                                                 false,},
                                    testfn:
                                        self::test::StaticTestFn(::__test_reexports::global_test),}];
}
