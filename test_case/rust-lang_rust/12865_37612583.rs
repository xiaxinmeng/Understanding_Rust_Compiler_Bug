 rust
#[crate_type = "lib"];
#[crate_id = "encode64#0.1"];
#[desc = "Encode hex messages as base64"];

extern crate serialize;

use serialize::base64::{ToBase64, STANDARD};
use serialize::hex::{FromHex};

/// given a string, convert that string to a base64 encoded string.
pub fn encode(to_encode: ~str) -> ~str {
    to_encode.from_hex().unwrap().to_base64(STANDARD)
}

#[test]
fn using_encode(){
    let input  = ~"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    let result = encode(input);
    assert_eq!(result, expected.to_owned());
}
