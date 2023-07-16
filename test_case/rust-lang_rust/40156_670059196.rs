rust
#[derive(Clone)]
pub struct Reservation {
    ...
    plunger: Arc<Sender<ResOrShutdown>>,
    ...
}

unsafe impl Send for Reservation {}
