rust
struct Foo {
    x: u32,
    y: u32
}

fn main() {
    let foos: &mut [Foo] = &mut [Foo { x: 0, y: 0 }, Foo { x: 0, y: 0 }];
    let p = &mut foos[0];
    let r = &mut p.x;
    let q = &mut foos[0].y;
    *q += 1;
    *r += 1;
}
