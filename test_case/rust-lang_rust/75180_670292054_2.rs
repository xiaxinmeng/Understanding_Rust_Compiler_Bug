rust
impl dyn Error {
    pub fn as_error(self: &Box<Self>) -> Compat<&Self>;

    pub fn into_error(self: Box<Self>) -> Compat<Box<Self>>;
}

pub struct Compat<T>(T);

impl<'a, T: ?Sized + Error> Error for Compat<&'a T>;

impl<T: ?Sized + Error> Error for Compat<Box<T>>;
