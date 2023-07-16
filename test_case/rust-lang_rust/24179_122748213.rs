 rust
pub trait DecoderState {
    type Error;
}

pub trait Decodable<D: DecoderState> {
    // Setting the default type below causes the ICE.
    // Using the statement:
    //   type Error;
    // (no type assignment) does not produce an ICE.
    type Error = D::Error;

    fn decode(d: &mut D) -> Result<Self, Self::Error>;
}

struct IAmADecoder;

enum IAmADecoderError {
    BadThings,
}

impl DecoderState for IAmADecoder {
    type Error = IAmADecoderError;
}

struct IAmDecodable;

impl<D: DecoderState> Decodable<D> for IAmDecodable {
    type Error = D::Error;

    fn decode(d: &mut D) -> Result<Self, Self::Error> {
        Ok(IAmDecodable)
    }
}

fn main() {
    let mut d = IAmADecoder;
    IAmDecodable::decode(&mut d);
}
