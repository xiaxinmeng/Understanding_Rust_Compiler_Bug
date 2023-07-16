
error: cannot use `{{borrowed_path}}` because it was mutably borrowed
{{snippet
    primary borrow_span
    label borrow_span "borrow of `{{borrowed_path}}` occurs here"
    label use_span "use of borrowed `{{borrowed_path}}`"}}
