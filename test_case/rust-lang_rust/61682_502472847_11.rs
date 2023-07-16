rust
Enum::Variant::<TypeArgs> // OK (1).
Enum::<TypeArgs>::Variant // OK.
Alias::Variant::<TypeArgs> // ERROR.
Alias::<TypeArgs>::Variant // OK.
