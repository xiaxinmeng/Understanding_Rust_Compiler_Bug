rust
#[stable(feature = "rust1", since = "1.0.0")]
#[doc(override_notability)]
pub struct Box<
    T: ?Sized,
    #[unstable(feature = "allocator_api", issue = "32838")] A: Allocator = Global,
>(Unique<T>, A);
