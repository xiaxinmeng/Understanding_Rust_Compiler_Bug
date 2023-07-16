
error[E0407]: method `is_ascii_control` is not a member of trait `AsciiExt`
   --> libstd/ascii.rs:209:9
    |
209 |         fn is_ascii_control(&self) -> bool { self.is_ascii_control() }
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not a member of trait `AsciiExt`
...
218 |     delegating_ascii_ctype_methods!();
    |     ---------------------------------- in this macro invocation
