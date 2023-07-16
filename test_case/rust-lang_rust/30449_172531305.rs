
pub type Handler = Box<Fn(&PanicInfo) + 'static + Sync + Send>;
pub fn set_handler<F>(F) where F: FnOnce(Handler) -> Handler
