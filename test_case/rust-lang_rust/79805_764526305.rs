rust
let xor_checksum = message.bytes().reduce(|a, b| a ^ b);
