rust
use num::BigUint; // Cargo.toml: num = "0.1"

pub trait Protocol: Sized + Default {
    type PacketIndex: Into<u64> + Into<BigUint>;
}

pub fn decrypt_portion<P: Protocol>(index: P::PacketIndex) {
    let iv: BigUint = loop{};
    let len: usize = loop{};
    let iv = iv ^ (index.into() << 16);
    let iv = iv ^ (BigUint::from(1_u8) << (len * 8));
    let _iv: &[u8] = &iv.to_bytes_be()[1..len + 1];
}
