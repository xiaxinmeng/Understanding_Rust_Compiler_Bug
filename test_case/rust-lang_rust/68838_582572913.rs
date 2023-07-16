rust
compile_error!(error, "message"); // Non-fatal error
compile_error!(fatal_error, "message"); // Fatal error
compile_error!(warning, "message"); // Warning
compile_error!("message"); // Sugar for `compile_error!(error, "message")`
