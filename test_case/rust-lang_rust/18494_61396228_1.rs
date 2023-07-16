
$ rustc callback.rs
callback.rs:31:9: 31:15 error: the trait `core::ops::FnOnce<(&str,), ()>` is not implemented for the type `fn(&Foo, &str)`
callback.rs:31         invoke(self, "hi2u", Foo::callback);
                       ^~~~~~
callback.rs:31:9: 31:15 note: the trait `core::ops::FnOnce` must be implemented because it is required by `invoke`
callback.rs:31         invoke(self, "hi2u", Foo::callback);
                       ^~~~~~
error: aborting due to previous error
