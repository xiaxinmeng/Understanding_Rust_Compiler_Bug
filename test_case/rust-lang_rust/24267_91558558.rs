 rust
struct Foo { n0: i32, n1: i32 }

fn leak_1_brk() -> Foo {
    let ret;
    loop {
        ret = Foo { n0: { break }, n1: 22 };
    }
    ret
}

fn main() { }
