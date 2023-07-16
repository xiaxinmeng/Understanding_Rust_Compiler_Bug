rust
struct Encrypted<T>(Vec<u8>, PhantomData<T>);

fn encrypt<T: Into<Vec<u8>, K: Key>(payload: T, key: K) -> Encrypted<T> {
    Encrypted(encrypt_raw(payload.into(), key), ...)
}

impl<T: TryFrom<Vec<u8>> Encrypted<T> {
    fn try_decrypt<K: Key>(&self, key: K) -> Result<T> {
        decrypt_raw(...).map(T::try_from)
    }
}
