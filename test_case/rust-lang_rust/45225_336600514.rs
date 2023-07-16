
Thread 2 "rustc" received signal SIGSEGV, Segmentation fault.
0x00007ffff672ed20 in rustc_typeck::astconv::{{impl}}::ast_ty_to_ty ()
    at src/librustc_typeck/astconv.rs:1003
1003                hir::TySlice(ref ty) => {
