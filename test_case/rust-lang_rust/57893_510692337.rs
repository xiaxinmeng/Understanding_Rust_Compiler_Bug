rust
trait Foo { 
    fn method(&self) { }
}
impl<T: ?Sized> Foo for T { 
}
