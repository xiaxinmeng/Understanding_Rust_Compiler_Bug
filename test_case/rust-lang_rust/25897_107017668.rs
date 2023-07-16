 rust
/home/manishearth/Mozilla/rust/src/libstd/thread/scoped_tls.rs:75:13: 75:44 error: the trait `core::marker::Sync` is not implemented for the type `core::cell::UnsafeCell<u32>` [E0277]
/home/manishearth/Mozilla/rust/src/libstd/thread/scoped_tls.rs:75             ::std::thread::ScopedKey::new();
                                                                              ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/home/manishearth/Mozilla/rust/src/libstd/thread/scoped_tls.rs:66:1: 87:2 note: in expansion of scoped_thread_local!
/home/manishearth/Mozilla/rust/src/libstd/thread/scoped_tls.rs:262:9: 262:53 note: expansion site
/home/manishearth/Mozilla/rust/src/libstd/thread/scoped_tls.rs:75:13: 75:44 note: `core::cell::UnsafeCell<u32>` cannot be shared between threads safely
/home/manishearth/Mozilla/rust/src/libstd/thread/scoped_tls.rs:75             ::std::thread::ScopedKey::new();
                                                                              ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/home/manishearth/Mozilla/rust/src/libstd/thread/scoped_tls.rs:66:1: 87:2 note: in expansion of scoped_thread_local!
/home/manishearth/Mozilla/rust/src/libstd/thread/scoped_tls.rs:262:9: 262:53 note: expansion site
rustc: x86_64-unknown-linux-gnu/stage1/test/rbmltest-x86_64-unknown-linux-gnu
rustc: x86_64-unknown-linux-gnu/stage1/test/alloctest-x86_64-unknown-linux-gnu
/home/manishearth/Mozilla/rust/src/liballoc/lib.rs:82:12: 82:18 warning: unused or unknown feature, #[warn(unused_features)] on by default
/home/manishearth/Mozilla/rust/src/liballoc/lib.rs:82 #![feature(unique)]
