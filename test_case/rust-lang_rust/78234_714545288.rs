rust
use futures::{future, select}; // 0.3.6

pub enum Foo {
    Bar,
    Baz,
}

/* this function does produce warnings
fn match_foo(foo: Foo) -> i32 {
    match foo {
       Bar => 1,
        _ => unreachable!("why you do this rust?"),
    }
}
*/

pub async fn asdf(foo: Foo) {
    let mut a = future::ready(4);

    select! {
        // but this doesn't
        a_res = a => match foo {
            Bar => 1,
            _ => 4,
            // _ => unreachable!("why you do this rust?"),
        },
    };
}

fn main() {
    futures::executor::block_on(asdf(Foo::Baz));
}
