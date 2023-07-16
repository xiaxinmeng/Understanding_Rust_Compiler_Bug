 rust
impl<S: 'static + Serialize, F> Serialize for F where F: 'static + for<'c, 'd> Fn(&'c Record<'d>) -> S
