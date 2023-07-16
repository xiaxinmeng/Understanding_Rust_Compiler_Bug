
14498.rs:6:5: 6:8 error: cannot assign to `**b` because it is borrowed
14498.rs:6     **b = 4i;
               ^~~
14498.rs:4:14: 4:15 note: borrow of `**b` occurs here
14498.rs:4     let c = &b;
                        ^
