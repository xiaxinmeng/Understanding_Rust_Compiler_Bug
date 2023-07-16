rust
pub trait Action {
    type Output: FnOnce();
    fn run(self) -> Self::Output;
}

impl<T: FnOnce(), F: FnOnce() -> T> Action for F {
    type Output = T;
    fn run(self) -> Self::Output {
        self()
    }
}

fn retry<A: Action>(action: A) -> impl FnOnce() {
    action.run()
}
//Note: when specifying the associate type explicitly, the borrow checker complained.
//fn retry<F:FnOnce(),A: Action<Output=F>>(action: A) -> impl FnOnce() {
//    action.run()
//}

/*
The `nested` closure avoids the check of its lifetime here, if:
- the `nasted` closure is nested into the `action` closure, and
- the `action` closure is passed into the `retry` function, and
- the `retry` function take a generic by the `Action` trait argument, and
- the `Action` trait is implemented for an `Fn*` trait.

As a result, we get arbitrary values in variables and at best SIGSEGV.
*/
fn main() {
    let mut core = Vec::new();
    for i in &[1, 2, 3, 4, 5] {
        println!("outer: {}", i);
        let f = move || {
            println!("inner: {}", i);
        };
        let action = move || {
            || f() // The `nested` closure
        };
        core.push(retry(action));
    }
    (core.remove(0)())
}
