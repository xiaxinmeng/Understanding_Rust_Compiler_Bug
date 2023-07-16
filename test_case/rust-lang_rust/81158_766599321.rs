plain
    Checking rustfix v0.5.1
error[E0283]: type annotations needed
   --> src/cargo/util/config/de.rs:530:63
    |
530 |                 seed.deserialize(Tuple2Deserializer(1i32, env.as_ref()))
    |                                                           |   |
    |                                                           |   |
    |                                                           |   cannot infer type for type parameter `T` declared on the trait `AsRef`
    |                                                           this method call resolves to `&T`
    |
    = note: cannot satisfy `std::string::String: AsRef<_>`
help: use the fully qualified path for the potential candidates
    |
530 |                 seed.deserialize(Tuple2Deserializer(1i32, <std::string::String as AsRef<OsStr>>::as_ref(env)))
    |                                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
530 |                 seed.deserialize(Tuple2Deserializer(1i32, <std::string::String as AsRef<std::path::Path>>::as_ref(env)))
    |                                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
530 |                 seed.deserialize(Tuple2Deserializer(1i32, <std::string::String as AsRef<str>>::as_ref(env)))
    |                                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
530 |                 seed.deserialize(Tuple2Deserializer(1i32, <std::string::String as AsRef<[u8]>>::as_ref(env)))

error: aborting due to previous error

For more information about this error, try `rustc --explain E0283`.
