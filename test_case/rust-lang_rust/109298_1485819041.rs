rs
pub fn capture_slice(x: [u8; 3]) {
    || {
        let [_x @ ..] = x;
    };
}
