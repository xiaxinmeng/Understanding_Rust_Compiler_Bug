plain
    Checking env_logger v0.9.0
    Checking color-eyre v0.6.2
    Checking measureme v10.1.0
    Checking ui_test v0.5.0
error[E0615]: attempted to take value of method `ident` on type `&&ModChild`
    |
    |
105 |             .filter(move |item| item.ident.name.as_str() == name)
    |
help: use parentheses to call the method
    |
    |
105 |             .filter(move |item| item.ident(_).name.as_str() == name)


error[E0615]: attempted to take value of method `res` on type `&ModChild`
    |
    |
106 |             .map(move |item| item.res.def_id())
    |
help: use parentheses to call the method
    |
    |
106 |             .map(move |item| item.res(_).def_id())

For more information about this error, try `rustc --explain E0615`.
error: could not compile `miri` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
