 rust
let mut pool = Pool(os::cpu_count());
let a = pool.spawn(foo);
let b = pool.spawn(bar);
let result = (a.join(), b.join());
