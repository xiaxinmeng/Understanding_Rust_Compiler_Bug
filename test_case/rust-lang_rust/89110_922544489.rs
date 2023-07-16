plain
   Compiling libz-sys v1.1.3
error[E0027]: pattern does not mention field `span`
   --> src/tools/clippy/clippy_utils/src/lib.rs:842:17
    |
842 |           if let [Adjustment {
    |  _________________^
843 | |             kind: Adjust::Deref(_) | Adjust::Borrow(AutoBorrow::Ref(..)),
844 | |             target,
845 | |         }, ref adjust @ ..] = *cx
    | |_________^ missing field `span`
help: include the missing field in the pattern
    |
    |
844 |             target, span }, ref adjust @ ..] = *cx
    |                   ~~~~~~~~
help: if you don't care about this missing field, you can explicitly ignore it
    |
844 |             target, .. }, ref adjust @ ..] = *cx

    Checking getrandom v0.2.0
    Checking dirs-sys-next v0.1.2
    Checking num_cpus v1.13.0
