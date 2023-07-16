rust
struct S<T> {
    t : T,
    s : Box<S<T>>
}

fn f(x : S<u32>) { }

fn main () { }
