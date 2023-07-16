plain
    Checking rand v0.7.3
    Checking std v0.0.0 (/checkout/library/std)
    Checking core v0.0.0 (/checkout/library/core)
    Checking alloc v0.0.0 (/checkout/library/alloc)
error: invalid format string: expected `'}'`, found `'?'`
   --> library/core/tests/fmt/num.rs:225:30
    |
225 |     assert_eq!(format!("{:03o?}", b"Foo\0"), "[106, 157, 157, 000]");
    |                         -    ^ expected `}` in format string
    |                         because of this opening brace
    |
    |
    = note: if you intended to print `{`, you can escape it using `{{`

error: invalid format string: expected `'}'`, found `'?'`
   --> library/core/tests/fmt/num.rs:226:30
    |
226 |     assert_eq!(format!("{:07b?}", b"Foo\0"), "[1000110, 1101111, 1101111, 0000000]");
    |                         -    ^ expected `}` in format string
    |                         because of this opening brace
    |
    |
    = note: if you intended to print `{`, you can escape it using `{{`

error: invalid format string: expected `'}'`, found `'?'`
   --> library/core/tests/fmt/num.rs:231:28
    |
231 |     assert_eq!(format!("{:e?}", &[1, 20, 300]), "[1e0, 2e1, 3e2]");
    |                         -  ^ expected `}` in format string
    |                         because of this opening brace
    |
    |
    = note: if you intended to print `{`, you can escape it using `{{`

error: invalid format string: expected `'}'`, found `'?'`
   --> library/core/tests/fmt/num.rs:232:28
    |
232 |     assert_eq!(format!("{:E?}", &[1, 20, 300]), "[1E0, 2E1, 3E2]");
    |                         -  ^ expected `}` in format string
    |                         because of this opening brace
    |
    |
    = note: if you intended to print `{`, you can escape it using `{{`

error: invalid format string: expected `'}'`, found `'?'`
   |
   |
27 |     assert_eq!(format!("{:p?}", data), format!("[{}]", data.map(|x| format!("{x:p}")).join(", ")));
   |                         -  ^ expected `}` in format string
   |                         because of this opening brace
   |
   |
   = note: if you intended to print `{`, you can escape it using `{{`
error: could not compile `core` due to 5 previous errors
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:01:45
