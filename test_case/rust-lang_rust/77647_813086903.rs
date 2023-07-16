
#![feature(const_evaluatable_checked)]
#![allow(incomplete_features)]

const fn a() -> usize {
    0
}
struct B
where
    [(); a()]: ;
