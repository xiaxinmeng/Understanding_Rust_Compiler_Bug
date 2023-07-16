rust
#[derive(Copy, Clone)]
struct Wrap<T, S>(T, S);

fn main() {
    let s = Wrap(None, None);
    constrain::<i32, _>(s);
    constrain::<_, i32>(s);
    let z: Wrap<Option<i32>, Option<&str>> = s;
}

fn constrain<T, S>(_: Wrap<Option<T>, Option<S>>) {}
