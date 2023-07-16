rust
pub trait Future {
    fn run(self);
}

impl<F> Future for F where F: FnOnce() {
    fn run(self) {
        self();
    }
}

pub trait Action {
    type Output: Future;
    fn run(self) -> Self::Output;
}

impl<T: Future, F: FnOnce() -> T> Action for F {
    type Output = T;
    fn run(self) -> Self::Output {
        self()
    }
}

fn retry<A: Action>(action: A) -> impl Future {
    action.run()
}

struct Core<F: Future> {
    vec: Vec<F>,
}

impl<F: Future> Core<F> {
    pub fn spawn(&mut self, f: F) where F: Future + 'static {
        self.vec.push(f);
    }

    pub fn run(self) {
        for f in self.vec.into_iter() {
            f.run()
        };
    }
}

/*
The `nested` closure avoids the check of its lifetime here, if:
- the `nasted` closure is nested into the `action` closure, and
- the `action` closure is passed into the `retry` function, and
- the `retry` function take a generic by the `Action` trait argument, and
- the `Action` trait is implemented for an `Fn*` trait.

As a result, we get arbitrary values in variables and at best SIGSEGV.
*/
fn main() {
    let mut core = Core { vec: Vec::new() };
    for i in &[1, 2, 3, 4, 5] {
        println!("outer: {}", i);
        let f = move || {
            println!("inner: {}", i);
        };
        let action = move || {
            || f() // The `nested` closure
        };
        core.spawn(retry(action));
    }
    core.run();
}

