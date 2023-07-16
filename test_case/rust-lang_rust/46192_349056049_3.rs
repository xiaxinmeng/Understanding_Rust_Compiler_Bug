Rust
impl<'a, E> From<E> for EventArray where E: Into<Event>
impl<'a, E> From<&'a E> for EventArray where E: Into<Event> + Clone
