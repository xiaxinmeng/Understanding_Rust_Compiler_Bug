rust
let xor_checksum = message.bytes().fold_first(|a, b| a ^ b);
