rust
enum E {
  A(u32),
  B,
}

// I'm assuming e's type is E here, but I guess it could already be <E is E::A(_)>
let e = E::A(3);
let e: E is E::A(_) = e.try_into().unwrap();
let <E is E::A(_)>::A(n) = e;
assert_eq!(n, 3);

// or perhaps even:

let e = E::A(3);
let E::A(n) = e.try_into().unwrap();
assert_eq!(n, 3);
