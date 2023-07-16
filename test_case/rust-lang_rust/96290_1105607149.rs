plain
    Checking proc_macro v0.0.0 (/checkout/library/proc_macro)
    Checking unicode-width v0.1.8
    Checking getopts v0.2.21
    Checking test v0.0.0 (/checkout/library/test)
error[E0277]: the trait bound `Box<dyn OutputFormatter>: OutputFormatter` is not satisfied
    |
    |
269 |                 Some(ref mut log_output) => Some(&mut *log_output),
    |                                             ---- ^^^^^^^^^^^^^^^^ the trait `OutputFormatter` is not implemented for `Box<dyn OutputFormatter>`
    |                                             required by a bound introduced by this call
    |
    = note: required for the cast to the object type `dyn OutputFormatter`

