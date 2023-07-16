
error[E0271]: type mismatch resolving `<impl core::future::future::Future as core::future::future::Future>::Output == std::option::Option<(<Fut as core::future::future::Future>::Output, (std::option::Option<Fut>, std::option::Option<std::pin::Pin<std::boxed::Box<St>>>))>`
  --> src/future.rs:99:48
   |
99 | pub fn flatten_stream<Fut, St>(future: Fut) -> impl Stream<Item = Fut::Output>
   |                                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected type parameter, found associated type
   |
   = note: expected type `std::option::Option<(St, (std::option::Option<Fut>, std::option::Option<std::pin::Pin<std::boxed::Box<St>>>))>`
              found type `std::option::Option<(<Fut as core::future::future::Future>::Output, (std::option::Option<Fut>, std::option::Option<std::pin::Pin<std::boxed::Box<St>>>))>`
   = note: required because of the requirements on the impl of `futures_core::stream::Stream` for `futures_util::stream::unfold::Unfold<(std::option::Option<Fut>, std::option::Option<std::pin::Pin<std::boxed::Box<St>>>), [closure@src/future.rs:104:51: 118:6], impl core::future::future::Future>`
   = note: the return type of a function must have a statically known size

error: aborting due to previous error
