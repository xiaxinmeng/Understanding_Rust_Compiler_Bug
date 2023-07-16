
iss10876.rs:12:29: 12:38 error: cannot borrow `*p.0` as mutable more than once at a time
iss10876.rs:12             &mut Nat::S(box ref mut n) => p = &mut *n
                                           ^~~~~~~~~
iss10876.rs:12:29: 12:38 note: previous borrow of `*p.0` occurs here; the mutable borrow prevents subsequent moves, borrows, or modification of `*p.0` until the borrow ends
iss10876.rs:12             &mut Nat::S(box ref mut n) => p = &mut *n
                                           ^~~~~~~~~
iss10876.rs:15:2: 15:2 note: previous borrow ends here
iss10876.rs:7 fn test(x: &mut Nat) {
...
iss10876.rs:15 }
               ^
iss10876.rs:12:43: 12:54 error: cannot assign to `p` because it is borrowed
iss10876.rs:12             &mut Nat::S(box ref mut n) => p = &mut *n
                                                         ^~~~~~~~~~~
iss10876.rs:12:29: 12:38 note: borrow of `p` occurs here
iss10876.rs:12             &mut Nat::S(box ref mut n) => p = &mut *n
                                           ^~~~~~~~~
error: aborting due to 2 previous errors
