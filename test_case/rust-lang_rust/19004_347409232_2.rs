rust
let _a = &mut foo.a;
|| &mut foo.b; // cannot borrow `foo` as mutable more than once at a time
