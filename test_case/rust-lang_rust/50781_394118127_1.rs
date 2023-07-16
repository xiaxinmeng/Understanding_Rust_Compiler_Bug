rust
not { exists<T> { ObjOf(T, Foo), not { ObjOf(T, Bar) }, Implemented(T: Bar) } }
