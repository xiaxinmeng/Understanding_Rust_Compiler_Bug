
!!! (resolving module in lexical scope) module wasn't actually a module!
foo.rs:8:39: 8:52 error: unresolved name
foo.rs:8         &Cons(val, ~ref next_list) => IntList::tail(next_list),
                                               ^~~~~~~~~~~~~
foo.rs:8:39: 8:52 error: use of undeclared module `IntList`
foo.rs:8         &Cons(val, ~ref next_list) => IntList::tail(next_list),
                                               ^~~~~~~~~~~~~
!!! (resolving module in lexical scope) module wasn't actually a module!
foo.rs:8:39: 8:52 error: unresolved name `IntList::tail`.
foo.rs:8         &Cons(val, ~ref next_list) => IntList::tail(next_list),
                                               ^~~~~~~~~~~~~
error: aborting due to 3 previous errors
