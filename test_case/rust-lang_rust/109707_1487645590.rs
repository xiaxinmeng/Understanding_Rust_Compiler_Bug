
    Updating crates.io index
 Downloading crates ...
  Downloaded generic-array v0.14.6
  Downloaded crypto-common v0.1.6
  Downloaded cpufeatures v0.2.5
  Downloaded block-padding v0.3.2
  Downloaded aes v0.8.2
  Downloaded version_check v0.9.4
  Downloaded typenum v1.16.0
  Downloaded inout v0.1.3
  Downloaded cipher v0.4.3
  Downloaded cbc v0.1.2
   Compiling typenum v1.16.0
   Compiling version_check v0.9.4
   Compiling cfg-if v1.0.0
   Compiling cpufeatures v0.2.5
   Compiling generic-array v0.14.6
   Compiling block-padding v0.3.2
   Compiling crypto-common v0.1.6
   Compiling inout v0.1.3
   Compiling cipher v0.4.3
   Compiling aes v0.8.2
   Compiling cbc v0.1.2
   Compiling set2 v0.1.0 (/home/isis/code/set2)
warning: unused import: `BlockEncryptMut`
 --> src/challenge10.rs:3:36
  |
3 | use aes::cipher::{BlockDecryptMut, BlockEncryptMut, KeyIvInit};
  |                                    ^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

error[E0308]: mismatched types
   --> src/challenge10.rs:18:42
    |
18  |         .decrypt_padded_vec_mut::<Pkcs7>(ciphertext)
    |          ------------------------------- ^^^^^^^^^^ expected `&[u8]`, found `String`
    |          |
    |          arguments to this method are incorrect
    |
note: method defined here
   --> /home/isis/.cargo/registry/src/index.crates.io-6f17d22bba15001f/cipher-0.4.3/src/block.rs:558:8
    |
558 |     fn decrypt_padded_vec_mut<P: Padding<Self::BlockSize>>(
    |        ^^^^^^^^^^^^^^^^^^^^^^

error[E0308]: mismatched types
    --> src/challenge10.rs:22:28
     |
22   |     result.copy_from_slice(plaintext);
     |            --------------- ^^^^^^^^^
     |            |               |
     |            |               expected `&[_]`, found `Vec<u8>`
     |            |               help: consider borrowing here: `&plaintext`
     |            arguments to this method are incorrect
     |            here the type of `result` is inferred to be `[_]`
     |
     = note: expected reference `&[_]`
                   found struct `Vec<u8>`
note: method defined here
    --> /home/isis/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:3331:12
     |
3331 |     pub fn copy_from_slice(&mut self, src: &[T])
     |            ^^^^^^^^^^^^^^^

For more information about this error, try `rustc --explain E0308`.
warning: `set2` (lib) generated 1 warning
error: could not compile `set2` (lib) due to 2 previous errors; 1 warning emitted
