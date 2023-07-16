
fn and<U,F:FromError<E>>(self, res: Result<U, F>) -> Result<U,F>;
