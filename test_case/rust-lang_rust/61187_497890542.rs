rust
#![feature(async_await)]

struct Session;

impl Session {
    fn do_thing(&mut self) {
        unimplemented!()
    }
}

async fn frob(session: Session) {
    session.do_thing()
}

fn main() {}
