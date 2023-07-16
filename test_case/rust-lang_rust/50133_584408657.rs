
pub fn typical_function<W, E>(target_width: W) -> Result<(), MyError>
where
    W: TryInto<BitWidth, Error = E>,
    MyError: From<E>,
