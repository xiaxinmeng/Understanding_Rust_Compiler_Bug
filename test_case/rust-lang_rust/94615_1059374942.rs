plain
   Compiling hashbrown v0.12.0
   Compiling object v0.26.2
   Compiling std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
   Compiling addr2line v0.16.0
error[E0446]: private type `BacktraceSymbol` in public interface
    |
    |
155 |     pub symbols: Vec<BacktraceSymbol>,
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ can't leak private type
165 | struct BacktraceSymbol {
165 | struct BacktraceSymbol {
    | ---------------------- `BacktraceSymbol` declared as private
error: missing documentation for a struct field
   --> library/std/src/backtrace.rs:155:5
    |
    |
155 |     pub symbols: Vec<BacktraceSymbol>,
    |
    |
    = note: `-D missing-docs` implied by `-D warnings`
For more information about this error, try `rustc --explain E0446`.
error: could not compile `std` due to 2 previous errors
Build completed unsuccessfully in 0:00:21
