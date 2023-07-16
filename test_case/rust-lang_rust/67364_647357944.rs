rust
struct Example {
    field: String,
    func: Box<dyn Fn(&str) -> bool + 'static>
}

impl fmt::Debug for Example {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Example")
            .field("field", &self.field)
            .finish_non_exhaustive()
    }
}
