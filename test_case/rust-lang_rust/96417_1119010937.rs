rust
// Use exclusive reference (but downgrade I guess):
let tmp = a.b2();
// &mut reference to a is valid again:
a.run(tmp);
