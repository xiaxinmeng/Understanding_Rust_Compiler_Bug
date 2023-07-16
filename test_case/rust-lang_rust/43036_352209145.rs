rust
#[repr(transparent, align=16)]
struct Aes256Key([u8; 32]);

#[repr(transparent, align=16)]
struct AesNonce([u8; 16]);

// Keep this in sync with the C definition in aes.h
#[repr(C)]
struct Aes256Context {
    key: Aes256Key,
    ...
    nonce: AesNonce,
    ...
}

extern aes256Encrypt(context: &mut Aes256Context, ...);
