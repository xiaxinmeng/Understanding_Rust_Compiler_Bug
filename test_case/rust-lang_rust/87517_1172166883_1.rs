rust
#[derive(Default)]
pub enum MaybeOption<T> {
    #[default]
    Yes(Option<T>),
    No,
}
