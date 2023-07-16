 rust
struct MemReader<'self> {
    buf: ~[u8],
    reader: MemReader<'self>
}
