
2021-01-25T11:21:52.6405659Z error[E0283]: type annotations needed
2021-01-25T11:21:52.6409991Z    --> src/cargo/util/config/de.rs:530:63
2021-01-25T11:21:52.6410516Z     |
2021-01-25T11:21:52.6411160Z 530 |                 seed.deserialize(Tuple2Deserializer(1i32, env.as_ref()))
2021-01-25T11:21:52.6412405Z     |                                                           ----^^^^^^--
2021-01-25T11:21:52.6413157Z     |                                                           |   |
2021-01-25T11:21:52.6413766Z     |                                                           |   cannot infer type for type parameter `T` declared on the trait `AsRef`
2021-01-25T11:21:52.6419097Z     |                                                           this method call resolves to `&T`
2021-01-25T11:21:52.6419655Z     |
2021-01-25T11:21:52.6420581Z     = note: cannot satisfy `std::string::String: AsRef<_>`
2021-01-25T11:21:52.6421342Z help: use the fully qualified path for the potential candidates
2021-01-25T11:21:52.6421925Z     |
2021-01-25T11:21:52.6422721Z 530 |                 seed.deserialize(Tuple2Deserializer(1i32, <std::string::String as AsRef<OsStr>>::as_ref(env)))
2021-01-25T11:21:52.6423727Z     |                                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2021-01-25T11:21:52.6424649Z 530 |                 seed.deserialize(Tuple2Deserializer(1i32, <std::string::String as AsRef<std::path::Path>>::as_ref(env)))
2021-01-25T11:21:52.6425505Z     |                                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2021-01-25T11:21:52.6426385Z 530 |                 seed.deserialize(Tuple2Deserializer(1i32, <std::string::String as AsRef<str>>::as_ref(env)))
2021-01-25T11:21:52.6427202Z     |                                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2021-01-25T11:21:52.6428044Z 530 |                 seed.deserialize(Tuple2Deserializer(1i32, <std::string::String as AsRef<[u8]>>::as_ref(env)))
2021-01-25T11:21:52.6429155Z     |                                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2021-01-25T11:21:52.6429431Z 
2021-01-25T11:21:55.7044874Z error: aborting due to previous error
2021-01-25T11:21:55.7045355Z 
2021-01-25T11:21:55.7046778Z For more information about this error, try `rustc --explain E0283`.
2021-01-25T11:21:55.7855655Z error: could not compile `cargo`
