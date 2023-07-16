rust
fn encode_num<W: ExampleWriter>(writer: W) -> W::Error {
    encode_num(&writer);
    loop {}
}

trait ExampleWriter {
    type Error;
}

impl<T: ExampleWriter> ExampleWriter for &T {
    type Error = T::Error;
}

impl ExampleWriter for () {
    type Error = ();
}

fn main() {
    encode_num(&());
}
