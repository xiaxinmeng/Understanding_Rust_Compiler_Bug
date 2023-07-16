rust
type A = u8;
struct B(u8);
struct C(u8);
struct D(u8);

enum Msgs {
Specific { a: A, b: B, c: C, d: D},
// snip
}
assert_matches!(rx.next().unwrap(), Msgs::Specific { a, b: B(b), .. } => { assert_eq!(a, b); })
assert_matches!(rx.next().unwrap(), Msgs::Specific { b, c: C(c), .. } => { assert_eq!(b, c); })
..
