Rust
struct Foo {
    v: Vec<i32>
}

fn bar(c: bool, f: &Foo, g: &[i32]) {
    let x = if true {
        { match f {
            &Foo { ref v } => &v,
        } }
    } else {
        g
    };

    println!("{:?}", x);
}

fn main() { }
