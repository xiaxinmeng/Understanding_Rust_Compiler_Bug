 rust
#[crate_type = "lib"];
#[crate_id = "encode64#0.1"];
#[desc = "Encode hex messages as base64"];

extern crate serialize;

use serialize::base64::{ToBase64, STANDARD};
use serialize::hex::{FromHex};

pub fn encode(to_encode: &str) -> ~str {                                        
    let hexed = to_encode.from_hex();
    if hexed.is_ok() {
            hexed.unwrap().to_base64(STANDARD)                                  
    }
    else {
        ~"error found in hexing"
    }
}

#[test]
fn using_encode(){
    let input  = ~"49276d206b696c6c696e6720796f757220627261696e206c696b65206120\
706f69736f6e6f7573206d757368726f6f6d";
    let expected = ~"SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2\
hyb29t";
    let result = encode(input);
    assert_eq!(result, expected);
}

