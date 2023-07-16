
Oct 14 19:10:36.049 INFO kablam!    Compiling woodpecker v0.4.0 (file:///source)
Oct 14 19:10:37.943 INFO kablam! error[E0277]: the trait bound `*mut std::ops::Fn() + 'static: std::marker::Sync` is not satisfied in `record::imp::SyncRecord<'a>`
Oct 14 19:10:37.943 INFO kablam!    --> src/record/imp.rs:166:10
Oct 14 19:10:37.943 INFO kablam!     |
Oct 14 19:10:37.943 INFO kablam! 166 | impl<'a> Record for SyncRecord<'a> {
Oct 14 19:10:37.943 INFO kablam!     |          ^^^^^^ `*mut std::ops::Fn() + 'static` cannot be shared between threads safely
Oct 14 19:10:37.943 INFO kablam!     |
Oct 14 19:10:37.943 INFO kablam!     = help: within `record::imp::SyncRecord<'a>`, the trait `std::marker::Sync` is not implemented for `*mut std::ops::Fn() + 'static`
Oct 14 19:10:37.943 INFO kablam!     = note: required because it appears within the type `std::marker::PhantomData<*mut std::ops::Fn() + 'static>`
Oct 14 19:10:37.943 INFO kablam!     = note: required because it appears within the type `core::fmt::Void`
Oct 14 19:10:37.943 INFO kablam!     = note: required because it appears within the type `&'a core::fmt::Void`
Oct 14 19:10:37.943 INFO kablam!     = note: required because it appears within the type `std::fmt::ArgumentV1<'a>`
Oct 14 19:10:37.944 INFO kablam!     = note: required because it appears within the type `[std::fmt::ArgumentV1<'a>]`
Oct 14 19:10:37.944 INFO kablam!     = note: required because it appears within the type `&'a [std::fmt::ArgumentV1<'a>]`
Oct 14 19:10:37.944 INFO kablam!     = note: required because it appears within the type `std::fmt::Arguments<'a>`
Oct 14 19:10:37.944 INFO kablam!     = note: required because it appears within the type `record::imp::SyncRecord<'a>`
Oct 14 19:10:37.944 INFO kablam! 
Oct 14 19:10:37.971 INFO kablam! error: aborting due to previous error
Oct 14 19:10:37.971 INFO kablam! 
Oct 14 19:10:37.983 INFO kablam! error: Could not compile `woodpecker`.
