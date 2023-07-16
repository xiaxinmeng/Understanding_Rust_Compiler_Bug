rust
#![feature(rustc_attrs)]
#![feature(const_if_match)]

#[rustc_mir(borrowck_graphviz_postflow="test.dot")]
const _: Option<&std::cell::Cell<i32>> = if false {
    Some(&std::cell::Cell::new(5))
} else {
    None
};
