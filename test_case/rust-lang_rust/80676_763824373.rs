
error[E0597]: `file` does not live long enough
   --> library/std/src/sys_common/backtrace.rs:230:24
    |
230 |     fmt::Display::fmt(&file.display(), fmt)
    |                        ^^^^----------
    |                        |
    |                        borrowed value does not live long enough
    |                        a temporary with access to the borrow is created here ...
231 | }
    | -
    | |
    | `file` dropped here while still borrowed
    | ... and the borrow might be used here, when that temporary is dropped and runs the destructor for type `impl core::fmt::Debug+core::fmt::Display`
    |
    = note: the temporary is part of an expression at the end of a block;
            consider forcing this temporary to be dropped sooner, before the block's local variables are dropped
help: for example, you could save the expression's value in a new local variable `x` and then make `x` be the expression at the end of the block
    |
230 |     let x = fmt::Display::fmt(&file.display(), fmt); x
    |     ^^^^^^^                                        ^^^
