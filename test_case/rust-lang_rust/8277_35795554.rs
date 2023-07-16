 rust
fn x<T>(t: T | int) {
    match t {
         T(t) => unimplemented!(),
         int(t) => unimplemented!(),
    }
}

x(42u);  // Fine. T = uint
x(42i);  // Not fine: T = <type error>
x::<bool>(42i);  // Fine. T = bool
