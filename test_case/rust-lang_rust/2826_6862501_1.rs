
  alt foo { bar(*x) { ... } }  // take-a-pointer to x
  alt foo { baz(&x) { ... } }  // match-existing-pointer, deref
  let quux : &x = &10;         // but ahh, type constructor == expr operator
  let *garply = 10;            // also type &int, pointing to 10, but even less intuitive?
