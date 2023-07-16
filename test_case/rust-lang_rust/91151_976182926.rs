plain
    Checking rand v0.7.3
    Checking core v0.0.0 (/checkout/library/core)
    Checking std v0.0.0 (/checkout/library/std)
    Checking alloc v0.0.0 (/checkout/library/alloc)
error: expected one of `.`, `;`, `?`, `else`, or an operator, found `arg`
  --> library/std/src/process/tests.rs:60:20
   |
60 |         shell_cmd()arg("-c").arg("read a").stdin(Stdio::piped()).spawn().unwrap();
   |                    ^^^ expected one of `.`, `;`, `?`, `else`, or an operator
error: unused import: `crate::os::unix::process::ExitStatusExt`
  --> library/std/src/process/tests.rs:57:9
   |
57 |     use crate::os::unix::process::ExitStatusExt;
57 |     use crate::os::unix::process::ExitStatusExt;
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `-D unused-imports` implied by `-D warnings`

error[E0599]: no method named `is_some` found for struct `process::ExitStatus` in the current scope
    --> library/std/src/process/tests.rs:169:20
169  |     assert!(status.is_some());
     |                    ^^^^^^^ method not found in `process::ExitStatus`
     |
    ::: library/std/src/process.rs:1428:1
    ::: library/std/src/process.rs:1428:1
     |
1428 | pub struct ExitStatus(imp::ExitStatus);
     | --------------------------------------- method `is_some` not found for this
For more information about this error, try `rustc --explain E0599`.
error: could not compile `std` due to 3 previous errors
warning: build failed, waiting for other jobs to finish...
error: build failed
