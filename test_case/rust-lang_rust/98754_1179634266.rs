rust
fn status(_client_status: &Client) -> i16 {
    200
}

fn main() {
    let client = Client;
    let g = move || match status(&client) {
        _status => yield,
    };
    assert_send(g);
}
