
/// A trait that is implemented for every type to conditionally determine whether it exposes type
/// information.
pub trait TryTypeInfo {
    /// The constant statically known type information for this type, or `None` if the type does not
    /// implement `TypeInfo`.
    const TRY_TYPE: Option<&'static Type>;
}

impl<T> TryTypeInfo for T {
    default const TRY_TYPE: Option<&'static Type> = None;
}

impl<T> TryTypeInfo for T
where
    T: TypeInfo,
{
    const TRY_TYPE: Option<&'static Type> = Some(&T::TYPE); //~ ERROR does not live long enough
}
