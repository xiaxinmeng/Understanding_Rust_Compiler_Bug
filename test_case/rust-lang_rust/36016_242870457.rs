
let ra = &u.a;
let rmb = &mut u.b; //~ ERROR cannot borrow `u` (via `u.b`) as mutable because `u` is also borrowed as immutable (via `u.a`)
