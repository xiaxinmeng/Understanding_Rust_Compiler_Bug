

error[E0277]: expected a `Fn<(char,)>` closure, found `char`
   --> src/lib.rs:1:36
    |
1   | const _: fn(&str, &char) -> bool = str::contains;
    |                                    ^^^^^^^^^^^^^ expected an `Fn<(char,)>` closure, found `char`
    |
    = help: the trait `Fn<(char,)>` is not implemented for `char`
    = note: required because of the requirements on the impl of `FnOnce<(char,)>` for `&char`
    = note: required because of the requirements on the impl of `Pattern<'_>` for `&char`
