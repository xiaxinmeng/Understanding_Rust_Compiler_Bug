
tst.rs:11:20: 11:29 error: cannot borrow `*p#0` as mutable more than once at a time
tst.rs:11             &S(box ref mut n) => p = &mut *n
                             ^~~~~~~~~
tst.rs:11:20: 11:29 note: previous borrow of `*p#0` occurs here; the mutable borrow prevents subsequent moves, borrows, or modification of `*p#0` until the borrow ends
tst.rs:11             &S(box ref mut n) => p = &mut *n
                             ^~~~~~~~~
tst.rs:14:2: 14:2 note: previous borrow ends here
tst.rs:6 fn test(x: &mut Nat) {
...
tst.rs:14 }
          ^
