rust
trait MyTrait {
    type MyType<T>: PartialEq<T>;
    //              ^^^^^^^^^ not what we wanted
}
