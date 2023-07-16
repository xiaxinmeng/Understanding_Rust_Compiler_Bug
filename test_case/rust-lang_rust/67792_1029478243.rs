rust
trait MyTrait {
    const fn f(v: i32) -> i32;
    fn g() {
        println!("nonconst");
    }
}

impl MyTrait for () {
    const fn f(v: i32) -> i32 {
        v + 1
    }
}

const fn f<T: MyTrait>() -> i32 {
    // can call T::f for any T, but not T::g
    T::f(0)
}
