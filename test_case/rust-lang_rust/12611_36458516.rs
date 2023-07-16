
trait A {
    fn foo();
}

trait B: A {
     fn bar();
}

trait C {
    fn test<T: A>(&self, x: T);
}

impl C for  int {
    fn test<T: B>(&self, x: T) {
         x.bar();
    }
}
