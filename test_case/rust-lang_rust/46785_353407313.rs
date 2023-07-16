rust
trait Trait<T> {}
struct Foo<U, V=i32>(U, V) 
    where U: Trait<V>;
______________________^ the trait `Trait<i32>` is not implemented for `U`
