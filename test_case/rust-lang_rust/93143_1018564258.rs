rs
// does not need to run, just tests by whether or not this compiles
fn _test<T>() {
    use std::panic::{RefUnwindSafe, UnwindSafe};

    fn all_auto_traits<T: Send + Sync + Unpin + UnwindSafe + RefUnwindSafe>() {}

    all_auto_traits::<std::iter::Empty<T>>();
    all_auto_traits::<std::hash::BuildHasherDefault<T>>();
    all_auto_traits::<std::future::Pending<T>>();
}
