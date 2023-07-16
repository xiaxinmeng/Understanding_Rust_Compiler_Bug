rs
type SubRef<'lt> = <Test as EnumRef>::Ref<'lt>;
match self {
    Self::Ref { a } => SubRef::Ref { a },
}
