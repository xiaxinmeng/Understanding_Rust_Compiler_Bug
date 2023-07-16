 rust
trait Tr<'a, T> {
    fn renew<'b: 'a>(self) -> &'b mut [T] where 'a: 'b;
}
...
