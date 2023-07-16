 rust
let Struct { .._ } = Struct { b: true, s: "hi", i: 23, f: 2.0 };

// Error, 'rest' cannot be named in structs
let Struct { x, ..end } = Struct { b: true, s: "hi", i: 23, f: 2.0 };

let Struct { s: s, i: i, .._ } = Struct { b: true, s: "hi", i: 23, f: 2.0 };
assert_eq!(s, "hi");
assert_eq!(i, 2.0);

let Struct { b, s, .._, f } = Struct { b: true, s: "hi", i: 23, f: 2.0 };
assert_eq!(b, true);
assert_eq!(s, "hi");
assert_eq!(f, 2.0);
