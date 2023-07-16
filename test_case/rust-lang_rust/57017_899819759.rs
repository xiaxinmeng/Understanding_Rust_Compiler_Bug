rust
#![feature(generators, generator_trait, negative_impls)]

struct Client;

impl !Sync for Client {}

fn status(_client_status: &Client) -> i16 {
    200
}

fn assert_send<T: Send>(_thing: T) {}

fn main() {
    let g = move || match status(&Client) {
        _status => yield,
    };
    assert_send(g);
}
