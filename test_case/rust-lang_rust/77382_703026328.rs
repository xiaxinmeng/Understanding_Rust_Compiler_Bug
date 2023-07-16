rust
enum Repr {
    Os(i32),
    Simple(ErrorKind),
    Custom(Box<Custom>),
}
