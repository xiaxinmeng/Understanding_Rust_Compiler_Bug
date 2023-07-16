rust
struct Sendable<T: Send>(T);
type MySendable<T> = Sendable<T>; // no error here!
