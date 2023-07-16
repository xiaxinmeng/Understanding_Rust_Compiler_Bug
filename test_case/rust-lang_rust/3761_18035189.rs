
struct Foo(~str);

fn main() {
    match Foo(~"hi") {
        msg @ Foo(_) => {}
    }
}
