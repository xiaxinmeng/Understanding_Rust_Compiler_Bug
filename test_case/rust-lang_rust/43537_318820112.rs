rust
    Option<Test>
==  enum {
        Some(enum {
            A(i32),   // <- discriminant = 0
            B(i32),   // <- discriminant = 1
        }),
        None,  // <- discriminant = 2
    }