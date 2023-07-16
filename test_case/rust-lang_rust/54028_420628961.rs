
#[repr(align(16))]
struct Foo {
    // ...
    field: PackedType, // Laid out at the offset 16.
}

#[packed] // so alignment = 1
struct PackedType { ... }
