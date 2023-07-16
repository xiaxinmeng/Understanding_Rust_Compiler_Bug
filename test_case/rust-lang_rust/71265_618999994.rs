rust
#![feature(raw_ref_op)]
struct Foo {
    x: u32,
    y: u32
}

fn main() {
    unsafe {
        let foos: &mut [Foo] = &mut [Foo { x: 0, y: 0 }, Foo { x: 0, y: 0 }];
        let p = &raw mut foos[0];
        let p = &raw mut (*p).x;
        let q = &raw mut foos[0].y;
        *q += 1;
        *p += 1;
    }
}
