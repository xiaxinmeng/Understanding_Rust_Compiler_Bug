
error[E0277]: `futures::stream::Iter<std::vec::IntoIter<{integer}>>` is not a future
 --> src/main.rs:4:5
  |
4 |     iter(vec![1, 2, 3])/*.collect::<Vec<_>>()*/.await;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `futures::stream::Iter<std::vec::IntoIter<{integer}>>` is not a future
  |
  = help: the trait `futures::Future` is not implemented for `futures::stream::Iter<std::vec::IntoIter<{integer}>>`
  = note: futures::stream::Iter<std::vec::IntoIter<{integer}>> must be a future or must implement `IntoFuture` to be awaited
  = note: required because of the requirements on the impl of `std::future::IntoFuture` for `futures::stream::Iter<std::vec::IntoIter<{integer}>>`
