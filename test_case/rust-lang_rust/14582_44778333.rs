 rust
trait MyTrait {
    fn act(_dummy: Option<Self>);
}

fn act<T: MyTrait>(val: &T) {
    MyTrait::act(None::<T>); // call T's act method
}
