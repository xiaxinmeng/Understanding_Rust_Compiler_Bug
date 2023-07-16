
warning[E0503]: cannot use `state` because it was mutably borrowed
    --> src/lib.rs:1033:33
     |
978  |     let mut machine = |cont: &mut bool, (i, c): (usize, char)| {
     |                       ---------------------------------------- borrow of `state` occurs here
...
982  |         state = match (state, whitespace, limit) {
     |         ----- borrow occurs due to use of `state` in closure
...
1033 |     while cont && match state { B | C => true, A => false } {
     |                                 ^ use of borrowed `state`
1034 |         machine(&mut cont, (fake_i, ' '));
     |         ------- borrow used here, in later iteration of loop
     |
     = warning: This error has been downgraded to a warning for backwards compatibility with previous releases.
             It represents potential unsoundness in your code.
             This warning will become a hard error in the future.
