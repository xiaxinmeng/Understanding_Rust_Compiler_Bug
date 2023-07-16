 rust
trait FooMarker { }
impl<T> FooMarker for T where T: Marker, T::Mark: Foo { }

impl<T> Foo for T where T: FooMarker {
    // ...
}
