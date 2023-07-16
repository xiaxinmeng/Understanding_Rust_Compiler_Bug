
error: there is no argument named `foo`
  --> C:\Users\david\dev\rust\src\test\ui\fmt\feature-gate-format-args-capture.rs:2:14
   |
LL |     format!("{foo}");                //~ ERROR: there is no argument named `foo`
   |              ^^^^^
   |
   = note: did you intend to capture a variable `foo` from the surrounding scope?
   = help: add `#![feature(format_args_capture)]` to the crate attributes to enable

error: aborting due to previous error
