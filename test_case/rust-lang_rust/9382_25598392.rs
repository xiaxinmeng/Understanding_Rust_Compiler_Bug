 rust
struct A<'self> {
    a: &'self [~int],
    b: ~[int]
}

fn main() {
    let _a = A{
        a:[],
        b:~[]
    };
}
