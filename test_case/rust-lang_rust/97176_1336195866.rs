plain
    Checking addr2line v0.17.0
error[E0308]: mismatched types
   --> library/std/src/sys/unix/process/process_common.rs:529:13
    |
528 | /         if self.stdin.is_some() {
529 | |             debug_command.field("stdin", &self.stdin)
    | |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `()`, found `&mut DebugStruct<'_, '_>`
    | |_________- expected this to be `()`
    |
help: consider using a semicolon here
    |
    |
529 |             debug_command.field("stdin", &self.stdin);
help: consider using a semicolon here
    |
530 |         };
    |          +
    |          +

error[E0308]: mismatched types
   --> library/std/src/sys/unix/process/process_common.rs:532:13
    |
531 | /         if self.stdout.is_some() {
532 | |             debug_command.field("stdout", &self.stdout)
    | |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `()`, found `&mut DebugStruct<'_, '_>`
    | |_________- expected this to be `()`
    |
help: consider using a semicolon here
    |
    |
532 |             debug_command.field("stdout", &self.stdout);
help: consider using a semicolon here
    |
533 |         };
    |          +
    |          +

error[E0308]: mismatched types
   --> library/std/src/sys/unix/process/process_common.rs:535:13
    |
534 | /         if self.stderr.is_some() {
535 | |             debug_command.field("stderr", &self.stderr)
    | |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `()`, found `&mut DebugStruct<'_, '_>`
    | |_________- expected this to be `()`
    |
help: consider using a semicolon here
    |
    |
535 |             debug_command.field("stderr", &self.stderr);
help: consider using a semicolon here
    |
536 |         };
    |          +
