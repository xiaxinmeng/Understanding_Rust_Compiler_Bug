rust
let a = [1, 2, 3];

let mut iter = a.iter().scan(1, |state, &x| {
    // each iteration, we'll multiply the state by the element
    *state = *state * x;

    // then, we'll yield the negation of the state
    Some(-*state)
});

assert_eq!(iter.next(), Some(-1));
assert_eq!(iter.next(), Some(-2));
assert_eq!(iter.next(), Some(-6));
assert_eq!(iter.next(), None);
