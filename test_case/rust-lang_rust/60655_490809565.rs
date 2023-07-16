rust
#![feature(gen_future)]
#![feature(generators)]
#![feature(async_await, await_macro, existential_type)]

use std::future::{self, Future};

async fn foo(__arg0: &u8) {
}

pub existential type ServeFut: Future<Output = ()>;

fn bar() -> ServeFut {
    ::std::future::from_generator(move || {
        let x = 5;

        {
            let mut pinned = foo(&x);
            loop {
                match future::poll_with_tls_context(unsafe {
                    std::pin::Pin::new_unchecked(&mut pinned)
                }) {
                    std::task::Poll::Ready(x) => {
                        break x;
                    }
                    _ => (),
                }
                yield ()
            }
        }
    })
}
fn main() {}
