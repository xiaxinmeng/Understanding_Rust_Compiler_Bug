plain
    Checking proc_macro v0.0.0 (/checkout/library/proc_macro)
    Checking unicode-width v0.1.8
    Checking getopts v0.2.21
    Checking test v0.0.0 (/checkout/library/test)
error: constant is never used: `BLACK`
   |
53 |     pub const BLACK: Color = 0;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   |
   = note: `-D dead-code` implied by `-D warnings`

error: constant is never used: `BLUE`
   |
57 |     pub const BLUE: Color = 4;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^


error: constant is never used: `MAGENTA`
  --> library/test/src/term.rs:58:5
   |
58 |     pub const MAGENTA: Color = 5;

error: constant is never used: `WHITE`
  --> library/test/src/term.rs:60:5
   |
