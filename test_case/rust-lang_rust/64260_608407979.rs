rust
x.ok();  // Result<T, E> → Option<T>
x.err(); // Result<T, E> → Option<E>

x.ok_or(y);            // Option<T> → Result<T, E>
x.ok_or_else(closure); // Option<T> → Result<T, E>

x.and(y); x.and_then(closure); // Result<T, E> → Result<U, E>
x.or(y); x.or_else(closure);   // Result<T, E> → Result<T, F>

x.and(y); x.and_then(closure); // Option<T> → Option<U>
x.or(y); x.or_else(closure);   // Option<T> → Option<T>
