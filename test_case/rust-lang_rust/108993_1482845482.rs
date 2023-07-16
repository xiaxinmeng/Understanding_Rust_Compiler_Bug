rust
struct Wrapper<T>(Option<T>);

impl<T> Wrapper<T> {
    fn something(&self) -> T {
        panic!()
    }
}

// using `FnOnce` bound to use `!` on stable.
fn infer<F: FnOnce() -> R, R>(_: Wrapper<R>) {}

pub fn with_never() {
    let x = Wrapper(None);

    if false {
        // currently doesn't warn as liveness analysis intentionally
        // skips `!`, won't warn with this PR
        //
        // warns that `y` is unused which just seems buggy?
        let y = x.something(); 
        let _ = y; 
    }

    infer::<fn() -> !, _>(x)
}

enum Void {}
pub fn with_void() {
    let x = Wrapper(None);

    if false {
        // currently warns that the assignment is unreachable,
        // will stop warning with this PR
        let y = x.something();
        let _ = y;
    }

    infer::<fn() -> Void, _>(x)
}
