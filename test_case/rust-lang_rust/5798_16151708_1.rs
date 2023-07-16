
borrowck-insert-during-each.rs:26:5: 26:6 note: prior loan as mutable granted here
borrowck-insert-during-each.rs:26   do f.foo |a| { //~ NOTE prior loan as mutable granted here
                                                             ^
borrowck-insert-during-each.rs:27:4: 27:5 error: loan of dereference of mutable & pointer as mutable conflicts with prior loan
borrowck-insert-during-each.rs:27     f.n.insert(*a); //~ ERROR conflicts with prior loan
                                                            ^
error: aborting due to previous error
