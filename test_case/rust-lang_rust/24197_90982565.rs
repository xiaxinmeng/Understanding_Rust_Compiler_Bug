
$ rustc h.rs
h.rs:4:14: 4:15 error: expected one of `:`, `;`, `=`, or `@`, found `[`
h.rs:4     let bytes[0] = (tag_size & 0x0FE00000 >> 21) as u8;
