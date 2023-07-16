
 = help: within `impl std::future::Future`, the trait `std::marker::Send` is not implemented for `std::sync::MutexGuard<'_, u32>`
   = note: required because it appears within the type `for<'r, 's> {&'r std::sync::Mutex<u32>, std::sync::MutexGuard<'s, u32>, impl std::future::Future, ()}`
   = note: required because it appears within the type `[static generator@src/main.rs:13:30: 16:2 x:&std::sync::Mutex<u32> for<'r, 's> {&'r std::sync::Mutex<u32>, std::sync::MutexGuard<'s, u32>, impl std::future::Future, ()}]`
   = note: required because it appears within the type `std::future::GenFuture<[static generator@src/main.rs:13:30: 16:2 x:&std::sync::Mutex<u32> for<'r, 's> {&'r std::sync::Mutex<u32>, std::sync::MutexGuard<'s, u32>, impl std::future::Future, ()}]>`
   = note: required because it appears within the type `impl std::future::Future`
   = note: required because it appears within the type `impl std::future::Future`
   = note: required because it appears within the type `for<'r> {impl std::future::Future, ()}`
   = note: required because it appears within the type `[static generator@src/main.rs:9:16: 11:2 for<'r> {impl std::future::Future, ()}]`
   = note: required because it appears within the type `std::future::GenFuture<[static generator@src/main.rs:9:16: 11:2 for<'r> {impl std::future::Future, ()}]>`
   = note: required because it appears within the type `impl std::future::Future`
   = note: required because it appears within the type `impl std::future::Future`
