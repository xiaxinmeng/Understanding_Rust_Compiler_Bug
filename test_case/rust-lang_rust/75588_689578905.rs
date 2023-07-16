rust
#![crate_name = "unstabled"]
#![feature(staged_api)]
#![unstable(feature = "thisisnotreal", issue = "27747")]

#[unstable(feature = "asdfasdfasdfa", issue = "27747")]
pub struct Foo<T: Sized + Clone> {
    #[unstable(feature = "asdfasdfasdfa", issue = "27747")]
    bytes: [T],
}

#[unstable(feature = "asdfasdfasdfa", issue = "27747")]
pub trait Join {
    #[unstable(feature = "asdfasdfasdfa", issue = "27747")]
    type Output;

    #[unstable(feature = "asdfasdfasdfa", issue = "27747")]
    fn join(slice: &Self) -> Self::Output;
}

#[unstable(feature = "asdfasdfasdfa", issue = "27747")]
impl<T: Sized + Clone> Join for Foo<T> {
    type Output = Vec<T>;

    fn join(slice: &Self) -> Vec<T> {
        unimplemented!()
    }
}
