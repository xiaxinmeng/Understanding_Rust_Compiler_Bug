rust
use crypto::hmac::Hmac;

fn hmac(key: &[u8], msg: &[u8]) -> MacResult {
    let mut hmac = Hmac::new(Sha256::new(), key);

    hmac.input(msg);
    let result = hmac.result();
    let _out = result.code();
    MacResult::new(_out)
}

fn get_signature(key: &[u8], signing_string: &str) -> Vec<u8> {
    hmac(key, signing_string.as_bytes()).code().to_vec()
}

let v = get_signature(/* fields */);
let s = String::from_utf_8(v);

println!("{}", s);
