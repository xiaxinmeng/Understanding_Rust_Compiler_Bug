 rust
pub pure fn slice<T: Copy>(v: &[const T], start: uint, end: uint) -> ~[T];
pub pure fn view<T>(v: &r/[T], start: uint, end: uint) -> &r/[T];
