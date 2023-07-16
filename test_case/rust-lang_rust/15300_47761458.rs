
fn my_add(lhs : &int, rhs : &int) -> int { *lhs + *rhs }; // the same as built-in add

let mut a = 1i;
let b1 = my_add({a = 10; &mut a}, {a = 100; &mut a}); // custom add, does't compile, borrow checker is not pleased
let b2 = *{a = 10; &mut a} + *{a = 100; &mut a}; // built-in add, suspiciously compiles, modifying and borrowing "a" while already borrowed
