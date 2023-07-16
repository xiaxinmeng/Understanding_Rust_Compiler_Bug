rust
#![feature(main)]
#![feature(box_syntax, structural_match)]
#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std as std;
extern crate diesel;

use std::rt;

struct MyLargeId(f64);

#[test]
pub fn can_serialize_large() {
    use diesel::types::ToSql;
    let mut st: Vec<u8> = Vec::new();
    let myid = MyLargeId(11f64);
    <MyLargeId as ToSql<diesel::types::Double, diesel::sqlite::Sqlite>>::to_sql(&myid, &mut st)
        .unwrap();
    {
        match (&st, &<[_]>::into_vec(box [0, 0, 0, 0, 0, 0, 38, 64])) {
            (left_val, right_val) => if !(*left_val == *right_val) {
                {
                    ::rt::begin_panic_fmt(
                        &::std::fmt::Arguments::new_v1(
                            {
                                static __STATIC_FMTSTR: &'static [&'static str] = &[
                                    "assertion failed: `(left == right)`\n  left: `",
                                    "`,\n right: `",
                                    "`",
                                ];
                                __STATIC_FMTSTR
                            },
                            &match (&left_val, &right_val) {
                                (__arg0, __arg1) => [
                                    ::std::fmt::ArgumentV1::new(__arg0, ::std::fmt::Debug::fmt),
                                    ::std::fmt::ArgumentV1::new(__arg1, ::std::fmt::Debug::fmt),
                                ],
                            },
                        ),
                        {
                            static _FILE_LINE_COL: (&'static str, u32, u32) =
                                ("tests/traits.rs", 41u32, 4u32);
                            &_FILE_LINE_COL
                        },
                    )
                }
            },
        }
    };
}
pub mod __test {
    extern crate test;
    #[main]
    pub fn main() -> () {
        test::test_main_static(TESTS)
    }
    const TESTS: &'static [self::test::TestDescAndFn] = &[];
}
