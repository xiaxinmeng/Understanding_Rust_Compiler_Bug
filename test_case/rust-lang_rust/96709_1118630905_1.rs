rust
type Result<T> = core::result::Result<T, Error>;

trait Iterator {
    type Item<'a>;
}
