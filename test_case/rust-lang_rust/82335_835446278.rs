rust
trait Trait {
    type Assoc;
}

struct Struct<T: Trait> {
    data: T::Assoc,
}

union Union<T> {
    data: Struct<T>
}
