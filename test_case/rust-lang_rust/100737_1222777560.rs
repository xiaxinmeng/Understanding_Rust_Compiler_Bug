rust
MyBuilder::new()
    .fallible_method(arg)
    .expect("arg is valid")
    .fallible_method_2(arg2)?
    .infallible_method()
    .build()
