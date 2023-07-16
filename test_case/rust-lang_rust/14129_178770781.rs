
#![feature(start)]

fn foo() {
        #[start]
        fn bar(_: isize, _: *const *const u8) -> isize { foo(); 0 }
}
