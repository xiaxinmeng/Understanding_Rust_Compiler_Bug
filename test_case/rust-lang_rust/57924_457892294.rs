rust
pub trait AeadCipher {
    fn new(key: &[u8]) -> Self;
}

pub trait BlockCipher {
    fn new(key: &[u8]) -> Self;
}

struct Processor<E> {
    block_cipher: E,
}

pub struct Gcm<E>(Processor<E>);

impl<E: BlockCipher> AeadCipher for Gcm<E> {
    fn new(key: &[u8]) -> Self {
        // Gcm::<E>(Processor::<E>::new(key))
        Self::<E>(Processor::<E>::new(key)) // ICE
    }
}

impl<E: BlockCipher> Processor<E> {
    fn new(key: &[u8]) -> Self {
        Self {
            block_cipher: E::new(key),
        }
    }
}
