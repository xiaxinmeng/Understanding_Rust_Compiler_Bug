rust
    let deserializers = DS { may_unwind: MayUnwind, name: "?".to_owned(), next: () };
    let deserializers = add(deserializers, "?".to_owned());
    let deserializers = add(deserializers, "?".to_owned());
    let deserializers = add(deserializers, "?".to_owned());
    let deserializers = add(deserializers, "?".to_owned()); // 0.6s
    let deserializers = add(deserializers, "?".to_owned()); // 0.6s
    let deserializers = add(deserializers, "?".to_owned()); // 1.0s
    let deserializers = add(deserializers, "?".to_owned()); // 2.2s
    let deserializers = add(deserializers, "?".to_owned()); // 8.0s
    let deserializers = add(deserializers, "?".to_owned()); // 34.6s
    // let deserializers = add(deserializers, "?".to_owned());
    // let deserializers = add(deserializers, "?".to_owned());
    // let deserializers = add(deserializers, "?".to_owned());
    