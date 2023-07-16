 rust
pub fn connect<T, B, E>(handle: &Handle, new_transport: T) -> Connecting<T::Future, Client<T::In, T::Out, B, E>>
    where T: NewTransport<Error = E>,
          T::In: Send + 'static,
          T::Out: Send + 'static,
          T::Item: Send + 'static,
          B: Stream<Item = T::BodyIn, Error = E> + Send + 'static,
E: From<Error<E>> + Send + 'static,
