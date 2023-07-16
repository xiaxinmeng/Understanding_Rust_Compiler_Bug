plain
   Compiling hashbrown v0.12.0
   Compiling object v0.26.2
   Compiling std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
   Compiling addr2line v0.16.0
error[E0446]: private type `BytesOrWide` in public interface
    |
    |
167 |     pub filename: Option<BytesOrWide>,
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ can't leak private type
...
172 | enum BytesOrWide {
    | ---------------- `BytesOrWide` declared as private
error: missing documentation for a struct field
   --> library/std/src/backtrace.rs:155:5
    |
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

For more information about this error, try `rustc --explain E0446`.
error: could not compile `std` due to 7 previous errors
Build completed unsuccessfully in 0:00:20
