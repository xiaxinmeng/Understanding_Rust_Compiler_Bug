
error[E0502]: cannot borrow `p` as immutable because it is also borrowed as mutable
  --> src/main.rs:9:20
   |
6  |     let q = id_ref(p); // error[E0502]: cannot borrow `p` as immutable because it is also borrowed as mutable
   |                    - implicit mutable reborrow occurs here
...
9  |     println!("{}", p);
   |                    ^ immutable borrow occurs here
10 |     q; // Keep `q` lives here
   |     - mutable reborrow later used here
