
error: invalid format string: expected `'}'`, found `'f'`
 --> src/main.rs:2:13
  |
2 |     format!(concat!("{", "  f"));
  |             ^^^^^^^^^^^^^^^^^^^ expected `}` in format string
  |
  = note: if you intended to print `{`, you can escape it using `{{`
  = note: this error originates in the macro `concat` (in Nightly builds, run with -Z macro-backtrace for more info)
