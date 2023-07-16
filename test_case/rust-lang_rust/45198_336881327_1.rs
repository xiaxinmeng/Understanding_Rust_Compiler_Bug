
Oct 14 23:03:28.806 INFO kablam! 228 | impl SyncSerialize for fmt::Arguments<'static> {}
Oct 14 23:03:28.806 INFO kablam!     |      ^^^^^^^^^^^^^ `*mut core::ops::Fn() + 'static` cannot be shared between threads safely
Oct 14 23:03:28.806 INFO kablam!     |
Oct 14 23:03:28.806 INFO kablam!     = help: within `core::fmt::Arguments<'static>`, the trait `core::marker::Sync` is not implemented for `*mut core::ops::Fn() + 'static`
Oct 14 23:03:28.806 INFO kablam!     = note: required because it appears within the type `core::marker::PhantomData<*mut core::ops::Fn() + 'static>`
Oct 14 23:03:28.807 INFO kablam!     = note: required because it appears within the type `core::fmt::Void`
Oct 14 23:03:28.807 INFO kablam!     = note: required because it appears within the type `&'static core::fmt::Void`
Oct 14 23:03:28.807 INFO kablam!     = note: required because it appears within the type `core::fmt::ArgumentV1<'static>`
Oct 14 23:03:28.807 INFO kablam!     = note: required because it appears within the type `[core::fmt::ArgumentV1<'static>]`
Oct 14 23:03:28.807 INFO kablam!     = note: required because it appears within the type `&'static [core::fmt::ArgumentV1<'static>]`
Oct 14 23:03:28.807 INFO kablam!     = note: required because it appears within the type `core::fmt::Arguments<'static>`
Oct 14 23:03:28.807 INFO kablam! 
Oct 14 23:03:28.961 INFO kablam! error: aborting due to previous error
Oct 14 23:03:28.962 INFO kablam! 
Oct 14 23:03:28.978 INFO kablam! error: Could not compile `slog`.
