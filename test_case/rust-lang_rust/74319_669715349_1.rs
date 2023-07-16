rust
let temp = v.len() / 2;
merge_sort(&mut v[..temp]);
merge_sort(&mut v[temp..]);
