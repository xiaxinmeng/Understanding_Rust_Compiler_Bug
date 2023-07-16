rust
trait T {
    type Item;

    fn f() where Self: T<Item = str> {}
}
