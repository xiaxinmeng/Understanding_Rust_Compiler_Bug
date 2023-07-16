
trait C {
    fn test<T: Eq + Eq>(&self);
}

impl C for  int {
    fn test<T: Eq + Eq >(&self) {}
}
