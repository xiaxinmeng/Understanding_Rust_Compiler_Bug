 rust
impl Clone for Server {
    fn clone(&self) -> Server {
        let mut handlers = self.routes.handlers.iter().map(|x| *x);
        Server { routes: Router { handlers: handlers.collect() } }
    }
}
