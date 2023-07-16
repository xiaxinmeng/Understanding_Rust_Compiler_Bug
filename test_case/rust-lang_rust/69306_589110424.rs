rust
struct _Bar<T>(T);

impl<T> _Bar<T> {
    fn _map1(x: u8) -> _Bar<u8> {
        Self(x)
    }
}
