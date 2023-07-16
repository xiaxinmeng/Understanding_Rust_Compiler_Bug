 rust
fn finished<T, E>(t: T) -> impl Future<Item=T, Error=E>
fn failed<T, E>(e: E) -> impl Future<Item=T, Error=E>
fn done<T, E>(e: Result<T, E>) -> impl Future<Item=T, Error=E>
