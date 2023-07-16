rust
struct S<T>(Option<T>);

struct TypeckResults<'a>(&'a ());

impl<T> S<T> {
    const fn map<F, U>(self, f: F) -> S<U> where F: ~const FnOnce(T) -> U + ~const Drop {
        match self {
            S(Some(x)) => S(Some(f(x))),
            S(None) => S(None),
        }
    }
}

const BAR: Option<fn(S<TypeckResults<'_>>) -> S<String>> = Some(|s| {
    s.map(|_| "bar".to())
});
