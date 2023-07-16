rust
doc_comment! {
    concat("Create a new ", stringify!($RcBox), "."),
    pub fn new(data: T) -> Self
    where
        T: Sized,
    {
        unsafe { $RcBox::from_unchecked($Rc::new(data)) }
    }
}
