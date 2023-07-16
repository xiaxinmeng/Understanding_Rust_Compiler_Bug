plain
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0061]: this function takes 0 arguments but 1 argument was supplied
    --> src/librustdoc/clean/mod.rs:1690:53
     |
1690 |                         let bound_predicate = bound.bound_atom(cx.tcx);
     |                                                     |
     |                                                     expected 0 arguments

error[E0061]: this function takes 0 arguments but 1 argument was supplied
error[E0061]: this function takes 0 arguments but 1 argument was supplied
    --> src/librustdoc/clean/mod.rs:1715:43
     |
1715 | ...                   bound.bound_atom(cx.tcx).skip_binder()
     |                             |
     |                             expected 0 arguments

error: aborting due to 2 previous errors
