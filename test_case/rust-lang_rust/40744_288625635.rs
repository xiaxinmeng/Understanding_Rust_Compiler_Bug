rust
// Pardon the typos, phone mode

fn c<U, T>(p: *mut *const U, ref: T) -> *mut T;
extern fn banana(){}
let myfn = *c(someptr, banana);
myfn()
