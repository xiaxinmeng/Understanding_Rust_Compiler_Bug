Rust

struct Test;

struct EventReader<R>(R);

impl<'a, R> TryFrom<&'a mut EventReader<R>> for Test {
    type Error = ();

    fn try_from(value: &'a mut EventReader<R>) -> Result<Self, Self::Error> {
        Other::try_from(value);
        Other::try_from(value);
        Ok(Test {})
    }
}

struct Other;

pub trait TryFrom<T>: Sized {
    /// The type returned in the event of a conversion error.
    type Error;

    /// Performs the conversion.
    fn try_from(value: T) -> Result<Self, Self::Error>;
}

impl<'a, R> TryFrom<&'a mut EventReader<R>> for Other {
    type Error = ();

    fn try_from(value: &'a mut EventReader<R>) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}

pub enum Infallible {}

#[cfg(error)]
impl<T, U> TryFrom<U> for T where T: From<U> {
    type Error = Infallible;

    fn try_from(value: U) -> Result<Self, Self::Error> {
        Ok(T::from(value))
    }
}

#[cfg(not(error))]
impl<'a, T> TryFrom<&'a str> for T where T: std::str::FromStr
{
    type Error = <T as std::str::FromStr>::Err;

    fn try_from(s: &'a str) -> Result<T, Self::Error> {
        std::str::FromStr::from_str(s)
    }
}

fn main() {}
