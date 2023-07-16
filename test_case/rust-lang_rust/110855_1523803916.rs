plain
    Checking diff v0.1.13
    Checking bytes v1.0.1
    Checking idna v0.2.0
    Checking rustc-workspace-hack v1.0.0
error[E0615]: attempted to take value of method `ident` on type `&&ModChild`
    |
    |
552 |             .filter(|item| item.ident.name == name)
    |
help: use parentheses to call the method
    |
    |
552 |             .filter(|item| item.ident(_).name == name)


error[E0615]: attempted to take value of method `res` on type `&ModChild`
    |
    |
553 |             .map(|child| child.res.expect_non_local())
    |
help: use parentheses to call the method
    |
    |
553 |             .map(|child| child.res(_).expect_non_local())

For more information about this error, try `rustc --explain E0615`.
error: could not compile `clippy_utils` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
