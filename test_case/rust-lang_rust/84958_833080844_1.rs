rust
use untrusted::{Slice};
struct Reader<'a> {
    pub input: Slice<'a>,
    pub i: usize,
}

#[inline(never)] // Required otherwise the test passes.
fn read_single_byte_value<'a>(input: &mut Reader<'a>) -> Slice<'a> {
    input.i = 1;
    input.input
}

#[inline(always)]
fn read(input: &mut Reader) {
    let value = read_single_byte_value(input);

    value.into_value().bytes.get(0).unwrap();

    let val = input.i;
    assert!(val == 1);
}

fn main() {
    let bytes = &[0x00];
    let slice = Slice { bytes };
    slice.into_value();
    let mut input = Reader {
        input: slice,
        i: 0,
    };
    read(&mut input)
}
