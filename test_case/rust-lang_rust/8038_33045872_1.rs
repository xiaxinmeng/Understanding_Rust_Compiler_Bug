
pub trait StreamingHash<S> {
    fn hash_stream(&self, state: &mut S);
}

impl<S: Streaming> StreamingHash<S> for u8 {
    fn hash_stream(&self, state: &mut S) {
        state.input([*self])
    }
}

pub fn sip_hash<T: StreamingHash<hash::SipState>>(v: &T) -> u64 {
    let mut state = hash::SipState::new(0, 0);
    v.hash_stream(&mut state);
    state.result_u64()
}
