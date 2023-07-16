rust
#[doc(hidden)]
pub trait CloneToAny {
    /// Clone `self` into a new `Box<CloneAny>` object.
    fn clone_to_any(&self) -> Box<CloneAny>;

    /// Clone `self` into a new `Box<CloneAny + Send>` object.
    fn clone_to_any_send(&self) -> Box<CloneAny + Send> where Self: Send;
}

impl<T: Any + Clone> CloneToAny for T {
    fn clone_to_any(&self) -> Box<CloneAny> {
        Box::new(self.clone())
    }

    fn clone_to_any_send(&self) -> Box<CloneAny + Send> where Self: Send {
        Box::new(self.clone())
    }
}

pub trait CloneAny: Any + CloneToAny { }

impl<T: Any + Clone> CloneAny for T { }

impl Clone for Box<CloneAny> {
    fn clone(&self) -> Box<CloneAny> {
        (**self).clone_to_any()
    }
}

impl Clone for Box<CloneAny + Send> {
    fn clone(&self) -> Box<CloneAny + Send> {
        (**self).clone_to_any_send()
    }
}
