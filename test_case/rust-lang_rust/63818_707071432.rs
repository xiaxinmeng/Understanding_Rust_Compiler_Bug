rust
struct Struct {
    unaliased_1: ...,
    aliased_field: AliasedCell<Field>,
    unaliased_2: ...,
    ...
}

let mut s = Struct { ... };
let at_field: *const Field = s.aliased_field.as_ptr();
stuff(&mut s);
other_stuff(&*at_field); // <- Would violate Stacked Borrows if it weren't for `AliasedCell`
