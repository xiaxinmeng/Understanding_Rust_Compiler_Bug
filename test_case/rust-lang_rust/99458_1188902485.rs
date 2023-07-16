diff
- xor_hasher(std::fs::File::open(&p_a)?.bytes(), sbox)
+ xor_hasher(std::io::BufReader::new(std::fs::File::open(&p_a)?).bytes(), sbox)
