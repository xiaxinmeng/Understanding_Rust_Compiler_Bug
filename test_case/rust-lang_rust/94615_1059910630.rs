plain
   Compiling addr2line v0.16.0
error: missing documentation for a struct field
   --> library/std/src/backtrace.rs:155:5
    |
155 |     pub symbols: Vec<BacktraceSymbol>,
    |
    |
    = note: `-D missing-docs` implied by `-D warnings`
error: missing documentation for a struct
   --> library/std/src/backtrace.rs:165:1
    |
165 | pub struct BacktraceSymbol {
165 | pub struct BacktraceSymbol {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^

error: missing documentation for a struct field
   --> library/std/src/backtrace.rs:166:5
    |
166 |     pub name: Option<Vec<u8>>,

error: missing documentation for a struct field
   --> library/std/src/backtrace.rs:167:5
    |
    |
167 |     pub filename: Option<BytesOrWide>,

error: missing documentation for a struct field
   --> library/std/src/backtrace.rs:168:5
    |
    |
168 |     pub lineno: Option<u32>,

error: missing documentation for a struct field
   --> library/std/src/backtrace.rs:169:5
    |
    |
169 |     pub colno: Option<u32>,

error: missing documentation for an enum
   --> library/std/src/backtrace.rs:172:1
    |
    |
172 | pub enum BytesOrWide {
    | ^^^^^^^^^^^^^^^^^^^^

error: missing documentation for a variant
   --> library/std/src/backtrace.rs:173:5
    |
173 |     Bytes(Vec<u8>),

error: missing documentation for a variant
   --> library/std/src/backtrace.rs:174:5
    |
