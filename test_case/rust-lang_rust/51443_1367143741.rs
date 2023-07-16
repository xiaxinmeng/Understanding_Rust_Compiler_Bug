rust
pub trait Trait {
    fn method(&self) where Self: Sync;
}
