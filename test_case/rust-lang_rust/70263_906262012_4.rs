rust
fn annotate<T, U, F>(f: F) -> F
where
    F: FnOnce(&mut T) -> &mut U,
{
    f
}

let m = Mutex::new((0, 0));

let mapper = annotate(|inner: &mut (_, _)| &mut inner.0);

let guard = MutexGuard::map(m.lock(), mapper);

// If you need to use some other type with a lifetime param, you need to write a new `annotate` fn
struct MutWrapper<'a, T>(&'a mut T);

fn annotate_wrapper<T, U, F>(f: F) -> F
where
    F: FnOnce(&mut T) -> MutWrapper<'_, U>
{
    f
}
