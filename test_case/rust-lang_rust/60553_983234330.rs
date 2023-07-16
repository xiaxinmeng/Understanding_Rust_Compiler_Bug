rust
// An enum with only unit variants.
// `repr` is optional
// Allows `as` casting
// This is the only form that allows explicit discriminants.
enum UnitOnly {
    Foo = 2,
    Bar = 4,
    Baz = 8,
}

// This is allowed on stable.
let x = UnitOnly::Foo as u8;

// An enum with fieldless struct and tuple variants.
// `repr` is optional
// Allows `as` casting
// Does not allow explicit discriminants.
enum Fieldless {
    Tuple(),
    Struct{},
    Unit,
}

// This is allowed on stable.
assert_eq!(Fieldless::Tuple() as u8, 0);
assert_eq!(Fieldless::Struct{} as u8, 1);
assert_eq!(Fieldless::Unit as u8, 2);

// Enums with fields do not allow `as` casting or explicit discriminants.
