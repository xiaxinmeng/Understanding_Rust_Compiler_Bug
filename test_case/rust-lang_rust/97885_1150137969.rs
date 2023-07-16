rs
pub fn encode(buffer: Vec<u8>, compress: bool, encrypt: Option<Aes256Gcm>) -> Vec<u8> {
    if compress {
        let zlib = ZlibEncoder::new(buffer, Compression::best());
        let compressed = zlib.finish().unwrap();
        if encrypt.is_some() {
            let nonce_bytes = random_number(96);
            let nonce = Nonce::from_slice(&nonce_bytes);
            let no_size: &[u8] = &vec_to_array(compressed);
            let encrypted = encrypt.unwrap().encrypt(nonce, no_size).unwrap();
            let result = Vec::new();
            result.append(&mut nonce_bytes);
            result.append(&mut encrypted);
            result
        } else {
            compressed
        }
    } else {
        if encrypt.is_some() {
            let nonce_bytes = random_number(96);
            let nonce = Nonce::from_slice(&nonce_bytes);
            let no_size: &[u8] = &vec_to_array(buffer);
            let encrypted = encrypt.unwrap().encrypt(nonce, no_size).unwrap();
            let result = Vec::new();
            result.append(&mut nonce_bytes);
            result.append(&mut encrypted);
            result
        } else {
            buffer
        }
    }
}
