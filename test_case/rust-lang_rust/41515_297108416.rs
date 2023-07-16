
[00:16:48] error: borrowed value does not live long enough
[00:16:48]    --> /checkout/src/libcore/char_private.rs:370:33
[00:16:48]     |
[00:16:48] 370 |   const NORMAL1: &'static [u8] = &[
[00:16:48]     |  _________________________________^
[00:16:48] 371 | |     0x5e, 0x22,
[00:16:48] 372 | |     0x7b, 0x05,
[00:16:48] 373 | |     0x03, 0x04,
[00:16:48] ...   |
[00:16:48] 521 | |     0x01, 0x86, 0x3f,
[00:16:48] 522 | | ];
[00:16:48]     | | -
[00:16:48]     | |_|
[00:16:48]     |   does not live long enough
[00:16:48]     |   temporary value only lives until here
[00:16:48]     |
[00:16:48]     = note: borrowed value must be valid for the static lifetime...
[00:16:48] 
[00:16:48] error: aborting due to 70 previous errors
[00:16:48] 
[00:16:48] error: Could not compile `core`.
