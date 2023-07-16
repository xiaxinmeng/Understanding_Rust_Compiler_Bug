rust
/// An iterator adapter that produces output as long as the underlying
/// iterator produces `Option::Some` values.
pub(crate) struct OptionShunt;

impl OptionShunt
{
    /// Process the given iterator as if it yielded a `T` instead of a
    /// `Option<T>`. Any `None` value will stop the inner iterator and
    /// the overall result will be a `None`.
    pub fn process<I, T, F, U, O>(iter: I, f: F) -> Option<U>
    where
        I: Iterator<Item = Option<T>>,
        O: Iterator<Item = Result<T, ()>>,
        F: FnMut(&mut ResultShunt<O, ()>) -> U,
    {
        ResultShunt::process(iter.map(|x| x.ok_or(())), f).ok()
    }
}
