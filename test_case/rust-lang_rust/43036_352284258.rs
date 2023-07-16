c
struct Foo {
    alignas(16) key[32];
    alignas(16) nonce[16];
};
