plain
   Compiling memchr v2.5.0
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.85
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: there is no argument named `pos`
    |
    |
185 |             write!(f, " at byte pos {pos}")?;
    |
    |
    = note: did you intend to capture a variable `pos` from the surrounding scope?
    = note: to avoid ambiguity, `format_args!` cannot capture variables when the format string is expanded from a macro

error: there is no argument named `len`
    |
    |
148 |     panic!("index out of bounds: the len is {len} but the index is {index}")
    |
    |
    = note: did you intend to capture a variable `len` from the surrounding scope?
    = note: to avoid ambiguity, `format_args!` cannot capture variables when the format string is expanded from a macro
error: there is no argument named `index`
   --> library/core/src/panicking.rs:148:12
    |
    |
148 |     panic!("index out of bounds: the len is {len} but the index is {index}")
    |
    |
    = note: did you intend to capture a variable `index` from the surrounding scope?
    = note: to avoid ambiguity, `format_args!` cannot capture variables when the format string is expanded from a macro
error: there is no argument named `msg`
    --> library/core/src/result.rs:1790:12
     |
     |
1790 |     panic!("{msg}: {error:?}")
     |
     |
     = note: did you intend to capture a variable `msg` from the surrounding scope?
     = note: to avoid ambiguity, `format_args!` cannot capture variables when the format string is expanded from a macro
error: there is no argument named `error`
    --> library/core/src/result.rs:1790:12
     |
     |
1790 |     panic!("{msg}: {error:?}")
     |
     |
     = note: did you intend to capture a variable `error` from the surrounding scope?
     = note: to avoid ambiguity, `format_args!` cannot capture variables when the format string is expanded from a macro
error: there is no argument named `index`
  --> library/core/src/slice/index.rs:53:12
   |
   |
53 |     panic!("range start index {index} out of range for slice of length {len}");
   |
   |
   = note: did you intend to capture a variable `index` from the surrounding scope?
   = note: to avoid ambiguity, `format_args!` cannot capture variables when the format string is expanded from a macro

error: there is no argument named `len`
   |
   |
53 |     panic!("range start index {index} out of range for slice of length {len}");
   |
   |
   = note: did you intend to capture a variable `len` from the surrounding scope?
   = note: to avoid ambiguity, `format_args!` cannot capture variables when the format string is expanded from a macro
error: there is no argument named `index`
  --> library/core/src/slice/index.rs:77:12
   |
   |
77 |     panic!("range end index {index} out of range for slice of length {len}");
   |
   |
   = note: did you intend to capture a variable `index` from the surrounding scope?
   = note: to avoid ambiguity, `format_args!` cannot capture variables when the format string is expanded from a macro

error: there is no argument named `len`
   |
   |
77 |     panic!("range end index {index} out of range for slice of length {len}");
   |
   |
   = note: did you intend to capture a variable `len` from the surrounding scope?
   = note: to avoid ambiguity, `format_args!` cannot capture variables when the format string is expanded from a macro
error: there is no argument named `index`
  --> library/core/src/slice/index.rs:99:12
   |
   |
99 |     panic!("slice index starts at {index} but ends at {end}");
   |
   |
   = note: did you intend to capture a variable `index` from the surrounding scope?
   = note: to avoid ambiguity, `format_args!` cannot capture variables when the format string is expanded from a macro
error: there is no argument named `end`
  --> library/core/src/slice/index.rs:99:12
   |
   |
99 |     panic!("slice index starts at {index} but ends at {end}");
   |
   |
   = note: did you intend to capture a variable `end` from the surrounding scope?
   = note: to avoid ambiguity, `format_args!` cannot capture variables when the format string is expanded from a macro

error: there is no argument named `oob_index`
    |
    |
109 |         panic!("byte index {oob_index} is out of bounds of `{s_trunc}`{ellipsis}");
    |
    |
    = note: did you intend to capture a variable `oob_index` from the surrounding scope?
    = note: to avoid ambiguity, `format_args!` cannot capture variables when the format string is expanded from a macro

error: there is no argument named `s_trunc`
    |
    |
109 |         panic!("byte index {oob_index} is out of bounds of `{s_trunc}`{ellipsis}");
    |
    |
    = note: did you intend to capture a variable `s_trunc` from the surrounding scope?
    = note: to avoid ambiguity, `format_args!` cannot capture variables when the format string is expanded from a macro

error: there is no argument named `ellipsis`
    |
    |
109 |         panic!("byte index {oob_index} is out of bounds of `{s_trunc}`{ellipsis}");
    |
    |
    = note: did you intend to capture a variable `ellipsis` from the surrounding scope?
    = note: to avoid ambiguity, `format_args!` cannot capture variables when the format string is expanded from a macro
error: could not compile `core` due to 14 previous errors
Build completed unsuccessfully in 0:04:03
