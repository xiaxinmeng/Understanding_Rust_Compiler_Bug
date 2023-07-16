rust
struct Session {
    opts: u8
}

fn main() {
    let sess: &Session = &Session { opts: 0 };
    (sess as *const Session as *mut Session).opts = 42;
}
