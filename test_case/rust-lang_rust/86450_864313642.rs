
warning: moving 16384 bytes
  --> ~/.cargo/registry/src/github.com-1ecc6299db9ec823/crc32fast-1.2.1/src/baseline.rs:38:19
   |
38 |             crc = CRC32_TABLE[0x0][buf[0xf] as usize]
   |                   ^^^^^^^^^^^ value moved from here
   |

warning: moving 16384 bytes
  --> ~/.cargo/registry/src/github.com-1ecc6299db9ec823/crc32fast-1.2.1/src/baseline.rs:39:19
   |
39 |                 ^ CRC32_TABLE[0x1][buf[0xe] as usize]
   |                   ^^^^^^^^^^^ value moved from here

...

warning: moving 65536 bytes
  --> ~/.cargo/registry/src/github.com-1ecc6299db9ec823/h2-0.3.3/src/hpack/huffman/mod.rs:92:35
   |
92 |         let (next, byte, flags) = DECODE_TABLE[self.state][input as usize];
   |                                   ^^^^^^^^^^^^ value moved from here
   |
   = note: `#[warn(large_assignments)]` on by default

warning: moving 4112 bytes
  --> ~/.cargo/registry/src/github.com-1ecc6299db9ec823/h2-0.3.3/src/hpack/huffman/mod.rs:50:29
   |
50 |         let (nbits, code) = ENCODE_TABLE[b as usize];
   |                             ^^^^^^^^^^^^ value moved from here

