rust
/// Providing metadata and conversions for primitive enum types.
pub trait ConvertableEnum: Sized {
    /// The underlying repr type of the enum.
    type Repr;

    /// Converts the enum into the underlying repr type.
    fn into_repr(self) -> Self::Repr;

    /// Tries to convert an enum from its underlying repr type.
    fn try_from_repr(from: Self::Repr) -> Result<Self, TryFromIntError>;
}

impl <C> TryFrom<u8> for C where C: ConvertableEnum<Repr = u8> + Sized {
    type Error = TryFromIntError;

    fn try_from(from: u8) -> Result<C, TryFromIntError> {
        C::try_from_repr(from)
    }
}
