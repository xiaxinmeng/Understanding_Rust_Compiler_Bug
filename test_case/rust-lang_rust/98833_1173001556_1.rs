
use once_cell::sync::Lazy;

pub static NULL_TLV: Lazy<asn1::Tlv<'static>> =
    Lazy::new(|| asn1::parse_single("\x05\x00").unwrap());
