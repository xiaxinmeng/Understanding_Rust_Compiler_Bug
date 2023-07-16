
trait A {
    fn foo<T: Eq + Ord>(&self);
}
