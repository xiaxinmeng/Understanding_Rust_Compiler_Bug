c
typedef uint8_t AES256Key[32];
typedef uint8_t AES128Key[16];
typedef uint8_t AESNonce[16];

struct Aes256Context {
    alignas(16) AES256Key key;
    alignas(16) AES256Nonce nonce;
};
