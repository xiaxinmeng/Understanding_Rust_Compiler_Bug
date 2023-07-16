plain
   Compiling proc_macro v0.0.0 (/checkout/library/proc_macro)
   Compiling unicode-width v0.1.8
   Compiling getopts v0.2.21
   Compiling test v0.0.0 (/checkout/library/test)
error[E0609]: no field `ignore_message` on type `&types::TestDesc`
   --> library/test/src/formatters/json.rs:124:41
    |
124 |                 if let Some(msg) = desc.ignore_message {
    |
    |
    = note: available fields are: `name`, `ignore`, `should_panic`, `compile_fail`, `no_run`, `test_type`
error[E0308]: mismatched types
   --> library/test/src/formatters/json.rs:125:21
    |
    |
124 |  /                 if let Some(msg) = desc.ignore_message {
125 |  |                     self.write_event(
126 | ||                         "test",
127 | ||                         desc.name.as_slice(),
128 | ||                         "ignored",
...   ||
...   ||
131 | ||                         Some(&*format!(r#""message": "{}""#, EscapedString(msg))),
    | ||_____________________^ expected `()`, found enum `Result`
133 |  |                 } else {
133 |  |                 } else {
134 |  |                     self.write_event("test", desc.name.as_slice(), "ignored", exec_time, stdout, None)
    |  |_________________- expected this to be `()`
    |
    = note: expected unit type `()`
                    found enum `Result<(), std::io::Error>`
---

error[E0308]: mismatched types
   --> library/test/src/formatters/json.rs:134:21
    |
124 | /                 if let Some(msg) = desc.ignore_message {
125 | |                     self.write_event(
126 | |                         "test",
127 | |                         desc.name.as_slice(),
...   |
134 | |                     self.write_event("test", desc.name.as_slice(), "ignored", exec_time, stdout, None)
135 | |                 }
    | |_________________- expected this to be `()`
    |
    = note: expected unit type `()`
    = note: expected unit type `()`
                    found enum `Result<(), std::io::Error>`
help: consider using a semicolon here
    |
134 |                     self.write_event("test", desc.name.as_slice(), "ignored", exec_time, stdout, None);
help: consider using a semicolon here
    |
135 |                 };
    |                  +
    |                  +
help: you might have meant to return this value
    |
134 |                     return self.write_event("test", desc.name.as_slice(), "ignored", exec_time, stdout, None);
    |                     ++++++                                                                                   +

error[E0609]: no field `ignore_message` on type `&types::TestDesc`
    |
    |
221 |             TestResult::TrIgnored => self.write_ignored(desc.ignore_message)?,
    |
    |
    = note: available fields are: `name`, `ignore`, `should_panic`, `compile_fail`, `no_run`, `test_type`
Some errors have detailed explanations: E0308, E0609.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `test` due to 4 previous errors
Build completed unsuccessfully in 0:00:38
