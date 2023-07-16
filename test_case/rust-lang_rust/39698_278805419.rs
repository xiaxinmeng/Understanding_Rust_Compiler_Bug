rust
enum T {
    T1(i32),
    T2(),
}

fn main() {
    match T::T1(123) {
        T::T1(a) | T::T2() => { println!("{:?}", a); }
    }
}
