rust
mem::swap(&mut s1, &mut s2);

// becomes, by language magic:

mem::swap(&mut s1.unaliased_1, &mut s2.unaliased_1);
Cell::swap(s1.aliased_field.as_cell_ref(), s2.aliased_field.as_cell_ref())
mem::swap(&mut s1.unaliased_2, &mut s2.unaliased_2);
// ...
