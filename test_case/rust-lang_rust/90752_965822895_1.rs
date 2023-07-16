rust
let mut foo = None;
let pfoo = &mut foo; // Mutable borrow here

match foo {
    Some(_) => return,
    None => (),
}
// `Some` variants are marked as uninitialized, despite the fact that a mutable borrow is outstanding.

*pfoo = Some(...); // Reinitialize `Some` variants, without creating a new mutable borrow.
