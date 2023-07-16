
match tail {
    &Cons(_, ~ref mut next) => { tail = next; },
    &Cons(_, ~Nil) => { break; },
}
