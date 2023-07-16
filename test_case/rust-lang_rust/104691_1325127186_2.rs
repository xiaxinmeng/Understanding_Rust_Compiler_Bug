rust
struct Thingy(u32);

fn foo<'c, 'b>(x: &'c mut &'b Thingy) -> impl FnOnce() + 'c {
    move || {
        println!("{}", x.0);
    }
}
