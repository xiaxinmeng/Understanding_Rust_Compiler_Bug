
error[E0631]: type mismatch in closure arguments
   --> src/main.rs:202:29
    |
202 |     git_ignore_files.iter().any(|f: File| f.is_excluded(&dir_entry.path()) )
    |                             ^^^ --------- found signature defined here
    |                             |
    |                             expected due to this
    |
    = note: expected closure signature `fn(&gitignore::File<'_>) -> _`
               found closure signature `for<'a> fn(gitignore::File<'a>) -> _`
note: required by a bound in `any`
   --> /rustc/4a04d086cac54a41517d5657b59d5fe2caca2d71/library/core/src/iter/traits/iterator.rs:2692:5
help: consider borrowing the argument
    |
202 |     git_ignore_files.iter().any(|f: &File| f.is_excluded(&dir_entry.path()) )
    |                                     +

For more information about this error, try `rustc --explain E0631`.
error: could not compile `dev-remenants` (bin "dev-remenants") due to previous error
