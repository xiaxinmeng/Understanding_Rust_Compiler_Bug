 rust
let i = 0;
let j = 1;
assert!(i != j, "Does this even make sense if i = j?");
let (min, max) = if i < j { (i, j) } else { (j, i) }
let (v1, v2) = v.split_at_mut(max);
v1[min].do_stuff(&mut v2[0])
