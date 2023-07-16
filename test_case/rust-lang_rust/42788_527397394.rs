rust
#![allow(bad_style)] // These would not be the final names

trait Read {
    // ...
    //! Added method:

    /// To be overriden by implementors of `ReadIntoUninit`
    #[inline]
    fn try_downcast_to_ReadIntoUninit (self: &'_ mut Self)
        -> Option<&'_ mut dyn ReadIntoUninit>
    {
        None
    }
}
