
trait S<T> {
    type Item = T;
}

struct MyStruct<S<_>> {
    feild1: S<u8>,
    feild2: S<i32>,
    feild3: S<f64>,
}
