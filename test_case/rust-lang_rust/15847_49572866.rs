 rust
#![feature(phase)]
#![no_std]
#![feature(globs)]
#[phase(plugin, link)]
extern crate std;
extern crate native;
use std::prelude::*;
mod foo {
    use std::prelude::*;
    mod bar {
        use std::prelude::*;
        #[test]
        fn bar_test() { }
        pub mod __test_reexports {
            use std::prelude::*;
            pub use foo::bar::bar_test;
        }
    }
    #[test]
    fn foo_test() { }
    pub mod __test_reexports {
        use std::prelude::*;
        pub use foo::bar;
        pub use foo::foo_test;
    }
}
#[test]
fn global_test() { }
pub mod __test_reexports {
    use std::prelude::*;
    pub use foo;
    pub use global_test;
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
                                        self::test::StaticTestFn(::__test_reexports::foo::__test_reexports::bar::__test_reexports::bar_test),},
          self::test::TestDescAndFn{desc:
                                        self::test::TestDesc{name:
                                                                 self::test::StaticTestName("foo::foo_test"),
                                                             ignore: false,
                                                             should_fail:
                                                                 false,},
                                    testfn:
                                        self::test::StaticTestFn(::__test_reexports::foo::__test_reexports::foo_test),},
          self::test::TestDescAndFn{desc:
                                        self::test::TestDesc{name:
                                                                 self::test::StaticTestName("global_test"),
                                                             ignore: false,
                                                             should_fail:
                                                                 false,},
                                    testfn:
                                        self::test::StaticTestFn(::__test_reexports::global_test),}];
}
