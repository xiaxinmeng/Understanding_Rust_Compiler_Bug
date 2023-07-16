 rust
fn main() {
    macro_rules! call_mut {
        ($e:expr, $i:expr) => {
            $e.call_mut()
        };
        ($e:expr, $i:expr, $($is:expr),+) => {
            call_mut!($e.call(), $($is),+)
            //          ^ this is the problem, it should be call_mut
        };
    }

    struct S;

    impl S {
        fn call(&self) -> &S { self }
        fn call_mut(&mut self) -> &mut S { self }
    }

    let mut s = S;
    let mut s2: &mut S = call_mut!(s, 1, 2);
}
