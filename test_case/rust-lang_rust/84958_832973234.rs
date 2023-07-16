rust
use untrusted::{Reader, Slice};

#[inline(never)] // Required otherwise the test passes.
fn read_single_byte_value<'a>(input: &mut Reader<'a>) -> Slice<'a> {
    input.read_bytes(1)
}

fn main() {
    let bytes = &[0x00];
    let slice = Slice { bytes };
    slice.read_all(|input| {
        let value = read_single_byte_value(input);

        let mut r2 = Reader::new(value);
        r2.read_byte();

        assert!(input.at_end());
    });
}

