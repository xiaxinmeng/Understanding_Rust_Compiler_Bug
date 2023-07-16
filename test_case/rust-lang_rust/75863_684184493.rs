rust
fn lift<T0, T1, T2>(f: fn(T0, T1) -> T2) -> impl Fn(Option<T0>, Option<T1>) -> Option<T2> {
    move |x, y| Some(f(x?, y?))
}

let add_opt = &lift(std::ops::Add::add);

assert_eq!(add_opt(Some(5), Some(8)), Some(13));
assert_eq!(add_opt(Some(13), None), None);

assert_eq!(vec![Some(3), Some(5), Some(8)].into_iter().fold(Some(0), add_opt), Some(16));
assert_eq!(vec![Some(3), None, Some(8)].into_iter().fold(Some(0), add_opt), None);
