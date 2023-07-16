
trait T {
    fn t(&self) -> &T where Self: Sized { self } // self, not &self
}
