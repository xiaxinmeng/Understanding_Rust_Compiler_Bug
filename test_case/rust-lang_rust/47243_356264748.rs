rust
...
// When encoding
let bytes: [u8; 16] = unsafe { mem::transmute([self.0.to_le(), self.1.to_le()]) };
...
// When decoding (slices cannot be destructured unfortunately).
let fingerprint: [u64; 2] = unsafe { mem::transmute(bytes) };
Ok(Fingerprint(u64::from_le(fingerprint[0]), u64::from_le(fingerprint[1])))
