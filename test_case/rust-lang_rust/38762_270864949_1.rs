rustc
let b;
let a = Foo { a: 1, b: 2, c: 3 };
b = foo(&a);
assert_eq!(a, b);
