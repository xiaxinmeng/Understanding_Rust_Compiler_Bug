rust
struct Slice(&'static [&'static [u8]]);
static MAP: Slice = Slice(&[
    b"CloseEvent" as &'static [u8],
]);
