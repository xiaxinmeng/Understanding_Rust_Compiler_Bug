
error[E0308]: mismatched types
  --> src/main.rs:41:5
   |
41 |     require_send(inner())
   |     ^^^^^^^^^^^^ one type is more general than the other
   |
   = note: expected type `std::ops::FnOnce<(std::result::Result<std::pin::Pin<std::boxed::Box<dyn core::future::future::Future<Output = ()> + std::marker::Send>>, ()>,)>`
              found type `std::ops::FnOnce<(std::result::Result<std::pin::Pin<std::boxed::Box<dyn core::future::future::Future<Output = ()> + std::marker::Send>>, ()>,)>`
