rust
//! Misusage
let x = UnsafeCell::new(42);
let p1 = &x;
let at_ft_mut = unsafe { &mut *p1.get() };
let p2 = &mut x;
let at_ft_mut_2 = p2.get_mut();
mem::swap(at_ft_mut, at_ft_mut_2);
