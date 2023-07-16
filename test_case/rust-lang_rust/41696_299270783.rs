Rust
// rustc -Copt-level=2 main.rs

extern "C" {
    static MAYBE: bool;
}

struct MayUnwind;

impl Drop for MayUnwind {
    fn drop(&mut self) {
        if unsafe { MAYBE } {
            panic!()
        }
    }
}

struct DS<U> {
    may_unwind: MayUnwind,
    name: String,
    next: U,
}

fn add<U>(ds: DS<U>, name: String) -> DS<DS<U>> {
    DS {
        may_unwind: MayUnwind,
        name: "?".to_owned(),
        next: ds,
    }
}

fn main() {
    let deserializers = DS { may_unwind: MayUnwind, name: "?".to_owned(), next: () };
    let deserializers = add(deserializers, "?".to_owned());
    let deserializers = add(deserializers, "?".to_owned());
    let deserializers = add(deserializers, "?".to_owned());
    let deserializers = add(deserializers, "?".to_owned());
    let deserializers = add(deserializers, "?".to_owned());
    let deserializers = add(deserializers, "?".to_owned());
    let deserializers = add(deserializers, "?".to_owned()); // 0.7s
    let deserializers = add(deserializers, "?".to_owned()); // 1.3s
    let deserializers = add(deserializers, "?".to_owned()); // 2.4s
    let deserializers = add(deserializers, "?".to_owned()); // 6.7s
    // let deserializers = add(deserializers, "?".to_owned()); // 26.0s
    // let deserializers = add(deserializers, "?".to_owned()); // 114.0s
}
