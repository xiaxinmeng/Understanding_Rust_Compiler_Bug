rust
// rustc -Copt-level=2 -Zincremental=. main.rs

struct DS<U> {
    name: String,
    next: U,
}

fn add<U>(ds: DS<U>, name: String) -> DS<DS<U>> {
    DS {
        name: "?".to_owned(),
        next: ds,
    }
}

fn main() {
    let deserializers = DS { name: "?".to_owned(), next: () };
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
