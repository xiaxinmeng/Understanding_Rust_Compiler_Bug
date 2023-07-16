 rust
    fn receive<F: Receive<T, E>+FnOnce(Result<T,E>)>(self, f: F)
