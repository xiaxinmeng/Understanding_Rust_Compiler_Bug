rust
#[doc(hidden)]
pub trait CloneToAny {
    /// Clone `self` into a new `Box<CloneAny>` object.
    fn clone_to_any(&self) -> Box<CloneAny>;

    /// Clone `self` into a new `Box<CloneAny + Send>` object.
    fn clone_to_any_send(&self) -> Box<CloneAny + Send> where Self: Send;

    /// Clone `self` into a new `Box<CloneAny + Sync>` object.
    fn clone_to_any_sync(&self) -> Box<CloneAny + Sync> where Self: Sync;

    /// Clone `self` into a new `Box<CloneAny + Send + Sync>` object.
    fn clone_to_any_send_sync(&self) -> Box<CloneAny + Send + Sync> where Self: Send + Sync;
}
