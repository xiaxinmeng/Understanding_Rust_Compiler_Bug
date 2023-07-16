rust
pub async fn wrapper<'a, Fun, Fut>(cb: Fun)
    where
        Fun: Fn(&'a mut [u8]) -> Fut ,
        Fut: Future<Output=()> + 'a
