
error[E0659]: `panic` is ambiguous
   --> src\main.rs:189:46
    |
189 |             (_, Number(a)) if a.is_zero() => panic!("no division by zero"),
    |                                              ^^^^^ ambiguous name
    |
    = note: ambiguous because of a conflict between a name from a glob import and an outer scope during import 
or macro resolution
    = note: `panic` could refer to a macro from prelude
note: `panic` could also refer to the macro imported here
   --> src\main.rs:47:5
    |
47  | use core::*;
    |     ^^^^^^^
    = help: consider adding an explicit import of `panic` to disambiguate
    = help: or use `crate::panic` to refer to this macro unambiguously

error[E0659]: `panic` is ambiguous
   --> src\main.rs:191:17
    |
191 |                 panic!("Infinity/Infinity division")
    |                 ^^^^^ ambiguous name
    |
    = note: ambiguous because of a conflict between a name from a glob import and an outer scope during import 
or macro resolution
    = note: `panic` could refer to a macro from prelude
note: `panic` could also refer to the macro imported here
   --> src\main.rs:47:5
    |
47  | use core::*;
    |     ^^^^^^^
    = help: consider adding an explicit import of `panic` to disambiguate
    = help: or use `crate::panic` to refer to this macro unambiguously
