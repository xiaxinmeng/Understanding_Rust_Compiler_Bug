 rust
// with implicit copy
let range = a..b;
v1.remove(range);
v2.remove(range);
// without implict copy (today)
let range = a..b;
v1.remove(range.clone());
v2.remove(range);
// however, I think it would be clearer to do
v1.remove(a..b);
v2.remove(a..b);
// which doesn't require an implicit copy
