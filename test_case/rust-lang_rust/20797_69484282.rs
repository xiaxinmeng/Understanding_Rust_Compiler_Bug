 rust
struct Peekable<I> where I: Iterator {
    _iter: I,
    _next: Option<<I as Iterator>::Item>,
}

fn main() {
    let mut iter = Vec::<i32>::new().into_iter();
    let next = iter.next();
    let _v = Peekable {
        _iter: iter,
        _next : next,
    };
}
