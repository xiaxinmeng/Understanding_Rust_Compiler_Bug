
#[deriving(Clone, DeepClone, Eq)]
pub struct Cell<T> {
    priv value: Option<T>
}
