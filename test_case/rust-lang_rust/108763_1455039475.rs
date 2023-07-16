rust
const A: [(); 5] = [(), (), (), (), ()];
const B: &[()] = unsafe { A.get_unchecked(3..1) };
dbg!(B.len()); // 18446744073709551614 today, uhoh
