 rust
struct Name(String);

fn apply<F>(f: F) where
    F: FnOnce() {

    f()
}

fn main() {
    let n = Name(String::new());
    let consume = || {
        let Name(i) = n;
    };

    apply(consume);
}
