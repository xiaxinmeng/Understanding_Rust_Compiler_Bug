
   = note: consider constraining the associated type `<impl futures_core::stream::TryStream as futures_core::stream::Stream>::Item` to `std::result::Result<(), futures_channel::mpsc::SendError>` like this:
  --> src/main.rs:14:13
   |
5  | pub fn create_stream() -> impl futures::TryStream<Ok=(), Error=mpsc::SendError, Item=std::result::Result<(), futures_channel::mpsc::SendError>> {
   |
   = note: or calling a method that returns `<impl futures_core::stream::TryStream as futures_core::stream::Stream>::Item`
