
error[E0744]: `?` is not allowed in a `const fn`
   --> library/core/src/iter/range.rs:438:19
    |
438 |             res = Step::forward_checked(res, 0x800)?;
    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
