
error: cannot use `{{borrowed_path}}` because it was mutably borrowed

Rust's mutability rules state that when a value is mutably borrowed with an `&mut` expression, that value can **only** be accessed through the resulting reference. In your code, an `&mut` borrow occurs here:

{{snippet primary borrow_span}}

But then the code later attempts to use `{{borrowed_path}}` here:

{{snippet primary use_span}}
