 rust
use std::mem;

trait T { fn foo(); }

struct TOption<'a> {
    v: Option<Box<T + 'a>>,
}

fn test<'a,'b>(a: TOption<'a>,
               b: TOption<'b>) {
    mem::drop::<TOption<'a>>(b);
    mem::drop::<TOption<'b>>(a);
}

fn main() { }
