
error[E0597]: `join` does not live long enough
   --> src/relation/peerjoin.rs:110:13
    |
110 |     let x = join.iter_all();
    |             ^^^^-----------
    |             |
    |             borrowed value does not live long enough
    |             argument requires that `join` is borrowed for `'static`
111 | //    assert_eq!(join.iter_all().count(), 3);
112 | } 
    | - `join` dropped here while still borrowed
