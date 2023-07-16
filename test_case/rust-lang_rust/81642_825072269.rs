rust
pub trait ConvertableEnum {
    type Repr;

    fn into_repr(self) -> Self::Repr;

    fn try_from_repr(from: Self::Repr) -> Result<Self, TryFromIntError>;
}

impl <C: ConvertableEnum> From<C> for C::Repr {
    fn from(convertable: C) -> Self {
        convertable.into_repr()
    }
}

impl <C: ConvertableEnum> TryFrom<C::Repr> for C {
    type Error = TryFromIntError;

    fn try_from(from: C::Repr) -> Result<C, Self::Error> {
        C::from_repr(from)
    }
}
