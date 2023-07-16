rust
#[derive(Default)]
pub enum Error<T> {
    #[default]
    Message(T),
    Other,
}
