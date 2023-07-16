rust
trait MyTrait {
    type MyType<T>: Iterator
    where
        T: Ord;
}
    
impl MyTrait for MyOtherType {
    type MyType<T> = MyIterator
    where
        T: Ord;
}
