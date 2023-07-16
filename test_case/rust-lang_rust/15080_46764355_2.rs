
main.rs:12:13: 21:10 error: non-exhaustive patterns: `&OtherType(_, _)` not covered
main.rs:12     let s = match t {
main.rs:13             &MemcheckAddr(n) => format!("Memcheck:Addr{:u}", n),
main.rs:14             &MemcheckLeak => format!("Memcheck:Leak"),
main.rs:15             &OtherType {
main.rs:16                 tool_name: ref tool_name,
main.rs:17                 suppression_type: ref suppression_type,
           ...
error: aborting due to previous error
