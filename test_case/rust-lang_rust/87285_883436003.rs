
LL | fn mk_int() -> usize { let i: isize = 3; return i; }
   |                -----                            ^ expected `usize`, found `isize`
   |                |
   |                expected `usize` because of return type
