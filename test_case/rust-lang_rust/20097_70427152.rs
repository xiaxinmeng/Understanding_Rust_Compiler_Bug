 .rs
#![feature(box_syntax)]
struct S1(u32);
trait T {}
impl T for S1 {}

#[cfg(broken)]
fn tst(foo: u8) -> Box<T + 'static> {
    match foo {
        0 => box S1(5),
        _ => box S1(6),
    }
}

#[cfg(works1)]
fn tst(foo: u8) -> Box<T + 'static> {
    match foo{
        0 => { box S1(5) },
        _ => { box S1(6) },
    }
}

#[cfg(works2)]
fn tst(foo: u8) -> Box<T + 'static> {
    match foo{
        0 => box S1(5) as Box<T + 'static>,
        _ => box S1(6) as Box<T + 'static>,
    }
}

fn main() {
    tst(0);
}
