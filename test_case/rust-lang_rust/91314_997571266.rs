rust
trait Read {}

impl Read for &[u8] {}

fn wants_read(_: impl Read) {}

fn main() {
    wants_read([0u8]);
}
