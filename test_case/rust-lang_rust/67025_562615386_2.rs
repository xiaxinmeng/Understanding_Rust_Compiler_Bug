
error[E0277]: `std::sync::mpsc::Sender<f32>` cannot be shared between threads safely
  --> src/bin/thead_pool_mpsc.rs:19:21
   |
19 |         thread_pool.spawn_ok(add_task(sender.clone(), i * 100));
   |                     ^^^^^^^^ `std::sync::mpsc::Sender<f32>` cannot be shared between threads safely
   |
   = help: the trait `std::marker::Sync` is not implemented for `std::sync::mpsc::Sender<f32>`
   = note: required because of the requirements on the impl of `std::marker::Send` for `&std::sync::mpsc::Sender<f32>`
   = note: required because it appears within the type `for<'r> {std::sync::mpsc::Sender<f32>, u64, &'r std::sync::mpsc::Sender<f32>, std::sync::mpsc::Sender<f32>, u64, std::time::Duration, impl core::future::future::Future, impl core::future::future::Future, ()}`
   = note: required because it appears within the type `[static generator@src/bin/thead_pool_mpsc.rs:9:54: 13:2 sender:std::sync::mpsc::Sender<f32>, i:u64 for<'r> {std::sync::mpsc::Sender<f32>, u64, &'r std::sync::mpsc::Sender<f32>, std::sync::mpsc::Sender<f32>, u64, std::time::Duration, impl core::future::future::Future, impl core::future::future::Future, ()}]`
   = note: required because it appears within the type `std::future::GenFuture<[static generator@src/bin/thead_pool_mpsc.rs:9:54: 13:2 sender:std::sync::mpsc::Sender<f32>, i:u64 for<'r> {std::sync::mpsc::Sender<f32>, u64, &'r std::sync::mpsc::Sender<f32>, std::sync::mpsc::Sender<f32>, u64, std::time::Duration, impl core::future::future::Future, impl core::future::future::Future, ()}]>`
   = note: required because it appears within the type `impl core::future::future::Future`
   = note: required because it appears within the type `impl core::future::future::Future`
