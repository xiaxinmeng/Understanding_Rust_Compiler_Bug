
[01:09:28] error[E0053]: method `late_callback` has an incompatible type for trait
[01:09:28]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rls-rustc-0.4.0/src/lib.rs:56:5
[01:09:28]    |
[01:09:28] 56 | /     fn late_callback(&mut self,
[01:09:28] 57 | |                      a: &CodegenBackend,
[01:09:28] 58 | |                      b: &getopts::Matches,
[01:09:28] 59 | |                      c: &Session,
[01:09:28] ...  |
[01:09:28] 65 | |         RustcDefaultCalls.late_callback(a, b, c, d, e, f, g)
[01:09:28] 66 | |     }
[01:09:28]    | |_____^ expected struct `rustc_metadata::cstore::CStore`, found trait rustc::middle::cstore::CrateStore
[01:09:28]    |
[01:09:28]    = note: expected type `fn(&mut ShimCalls, &dyn rustc_codegen_utils::codegen_backend::CodegenBackend, &getopts::Matches, &rustc::session::Session, &rustc_metadata::cstore::CStore, &rustc::session::config::Input, &std::option::Option<std::path::PathBuf>, &std::option::Option<std::path::PathBuf>) -> rustc_driver::Compilation`
[01:09:28]               found type `fn(&mut ShimCalls, &dyn rustc_codegen_utils::codegen_backend::CodegenBackend, &getopts::Matches, &rustc::session::Session, &dyn rustc::middle::cstore::CrateStore, &rustc::session::config::Input, &std::option::Option<std::path::PathBuf>, &std::option::Option<std::path::PathBuf>) -> rustc_driver::Compilation`
[01:09:28]
[01:09:28] error: aborting due to previous error
[01:09:28]
[01:09:28] For more information about this error, try `rustc --explain E0053`.
[01:09:28] [RUSTC-TIMING] rls_rustc test:false 1.737
[01:09:28] error: Could not compile `rls-rustc`.
