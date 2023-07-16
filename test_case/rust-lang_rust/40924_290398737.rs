rust
struct Foo {
    v: Vec<i32>
}

fn bar(f: &Foo, g: &[i32]) {
    let x = match true {
        true =>
            match f {
                &Foo { ref v } => &v,
            },
        false =>
            g,
    };

    println!("{:?}", x);
}

fn main() { }
