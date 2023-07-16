
struct HashEncoder<S> { state: S }

impl<S: Streaming> Encoder for HashEncoder<S> { ... }

trait HashEncodable<S: Streaming>: Encodable<HashEncoder<S>> { }

pub fn sip_hash<T: HashEncodable<hash::SipState>>(v: &T) -> u64 {
    let mut encoder = HashEncoder::new(hash::SipState::new(0, 0));
    v.encode(&mut encoder);
    encoder.state.result_u64()
}
