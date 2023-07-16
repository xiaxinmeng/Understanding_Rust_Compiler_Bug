rust
struct Client;

impl Client {
    fn post<T: std::ops::Add>(&self, _: T, _: T) {}
}

fn f() {
    let c = Client;
    post(c, ());
}
