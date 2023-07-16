
time, please rewrite code that relies on it
/opt/rust/src/libstd/lib.rs:116 #![feature(old_impl_check)]
                                           ^~~~~~~~~~~~~~
/opt/rust/src/libstd/thread_local/scoped.rs:122:51: 122:70 error: chained comparison operators require parentheses
/opt/rust/src/libstd/thread_local/scoped.rs:122                 marker: ::std::marker::PhantomData<::std::cell::Cell<$t>>,
                                                                                                  ^~~~~~~~~~~~~~~~~~~
/opt/rust/src/libstd/thread_local/scoped.rs:122:70: 122:72 error: unexpected token: `u32`
/opt/rust/src/libstd/thread_local/scoped.rs:122                 marker: ::std::marker::PhantomData<::std::cell::Cell<$t>>,
