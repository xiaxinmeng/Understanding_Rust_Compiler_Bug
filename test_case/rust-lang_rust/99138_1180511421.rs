plain
    Checking rand v0.7.3
    Checking core v0.0.0 (/checkout/library/core)
    Checking std v0.0.0 (/checkout/library/std)
    Checking alloc v0.0.0 (/checkout/library/alloc)
error: invalid format string: expected `'}'`, found `'?'`
   --> library/core/tests/fmt/num.rs:234:28
    |
234 |     assert_eq!(format!("{:e?}", &[1, 20, 300]), "[1e0, 2e1, 3e2]");
    |                         -  ^ expected `}` in format string
    |                         because of this opening brace
    |
    |
    = note: if you intended to print `{`, you can escape it using `{{`

error: invalid format string: expected `'}'`, found `'?'`
   --> library/core/tests/fmt/num.rs:235:28
    |
235 |     assert_eq!(format!("{:E?}", &[1, 20, 300]), "[1E0, 2E1, 3E2]");
    |                         -  ^ expected `}` in format string
    |                         because of this opening brace
    |
    |
    = note: if you intended to print `{`, you can escape it using `{{`

error: invalid format string: expected `'}'`, found `'?'`
   |
   |
27 |     assert_eq!(format!("{data:p?}"), format!("{data:p}"));
   |                         -      ^ expected `}` in format string
   |                         because of this opening brace
   |
   |
   = note: if you intended to print `{`, you can escape it using `{{`
error: could not compile `core` due to 3 previous errors
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:01:48
