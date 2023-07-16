 rust
#![feature(default_type_parameter_fallback)]

pub trait AllocResult<T = (), E = ()> {
    fn ok(value: T) -> Self;
    fn err(err: E) -> Self;
}

impl<T, E> AllocResult<T, E> for T {
    fn ok(value: T) -> T { value }
    fn err(_err: E) -> T { oom(); }
}

impl<T, E> AllocResult<T, E> for Result<T, E> {
    fn ok(value: T) -> Self { Ok(value) }
    fn err(err: E) -> Self { Err(err) }
}

impl<T> Vec<T> {
    pub fn with_capacity<T, R: AllocResult<Self> = Self>(cap: usize) -> R {
        let ptr = alloc::heap::allocate(...);
        if ptr.is_null() { R::err(()) } else { R::ok(Vec { ... }) }
    }

    pub fn push<R: AllocResult = ()>(&mut self, item: T) -> R {
        if self.needs_reallocate() {
            let ptr = alloc::heap::reallocate(...);
            if ptr.is_null() { return R::err(()); }
            ...
        }
        ...
        R::ok(())
    }
}

#[test]
fn test() {
    let vec: Vec<i32> = Vec::with_capacity(5); // aborts on OOM

    let mut vec: Vec<i32> = match Vec::with_capacity(5) { // returns `Err` on OOM
        Ok(vec) => vec,
        Err(_) => panic!("OOM"),
    };

    vec.push(1); // aborts on OOM

    match vec.push(1) { // returns `Err` on OOM
        Ok(_) => {}
        Err(_) => panic!("OOM"),
    }
}
