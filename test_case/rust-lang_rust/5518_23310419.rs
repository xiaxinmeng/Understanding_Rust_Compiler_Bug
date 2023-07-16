
#[crate_type = "lib"];

trait A<'self, T> {
    fn f(&mut self) -> &'self mut T;
    fn p() -> T;
}
