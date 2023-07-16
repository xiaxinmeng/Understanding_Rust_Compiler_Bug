rust
struct Foo {
    a: usize,
    b: &'static u32,
}

fn main() {
    futures::executor::block_on(async {
        // Swap the order here and it works
        let action = Foo {
            b: &42,
            a: async { 0 }.await,
        };
        
        println!("{:p}", action.b);
        // ^ This prints `0x0`

        async {}.await;
    });
}
