rust
pub enum Request {
    Resolve { url: String },
}

pub fn handle_event(event: Request) {
    (move || {
        let Request::Resolve { url: _url } = event;
    })();
}
