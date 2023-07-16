
impl<'a> FnMut<(&'a Context,)> for 'tcx {
    type Output = ();
    fn print () -> Self::Output{ }
}
