
error[E0008]: cannot bind by-move into a pattern guard
    --> librustc_borrowck/borrowck/mod.rs:1195:73
     |
1195 |                             (hir::Ty_::TyPtr(ref ty_ptr_mutability), Ok(snippet)) if ty_ptr_mutability.mutbl == bind_value_mut => {
     |                                                                         ^^^^^^^ moves value into pattern guard

error: aborting due to previous error

error: Could not compile `rustc_borrowck`.
