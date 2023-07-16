rust
#![feature(arbitrary_enum_discriminant)]

// An enum with fieldless struct and tuple variants.
// `repr` is required to specify explicit discriminants.
// Allows `as` casting
#[repr(u8)]
enum FieldlessExplicit {
    Tuple() = 1,
    Struct{} = 3,
    Unit = 9,
}

// arbitrary_enum_discriminant allows `as` casting these
assert_eq!(FieldlessExplicit::Tuple() as u8, 1);
assert_eq!(FieldlessExplicit::Struct{} as u8, 3);
assert_eq!(FieldlessExplicit::Unit as u8, 9);

// An enum with fields.
// `repr` is required to specify explicit discriminants.
// Does not allow direct `as` casting.
#[repr(u8)]
enum WithFields {
    Tuple(i32) = 1,
    Struct{f: i32} = 3,
    Unit = 5,
}

// let x = WithFields::Tuple(1) as u8;  // ERROR: casting not allowed
// It requires unsafe pointer casting to get the discriminant.
unsafe {
    assert_eq!(*(&WithFields::Tuple(123) as *const WithFields as *const u8), 1);
    assert_eq!(*(&WithFields::Struct { f: 4 } as *const WithFields as *const u8), 3);
    assert_eq!(*(&WithFields::Unit as *const WithFields as *const u8), 5);
}
