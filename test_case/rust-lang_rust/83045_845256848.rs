
thread 'rustc' panicked at 'Failed to get crate data for crate15', compiler/rustc_metadata/src/creader.rs:139:32
stack backtrace:
   0: rust_begin_unwind
             at /home/dennis/Projects/rust/rust/library/std/src/panicking.rs:515:5
   1: std::panicking::begin_panic_fmt
             at /home/dennis/Projects/rust/rust/library/std/src/panicking.rs:457:5
   2: rustc_metadata::creader::CStore::get_crate_data::{{closure}}
             at /home/dennis/Projects/rust/rust/compiler/rustc_metadata/src/creader.rs:139:32
   3: core::option::Option<T>::unwrap_or_else
             at /home/dennis/Projects/rust/rust/library/core/src/option.rs:427:21
   4: rustc_metadata::creader::CStore::get_crate_data
             at /home/dennis/Projects/rust/rust/compiler/rustc_metadata/src/creader.rs:137:21
   5: rustc_metadata::creader::CrateLoader::verify_no_stable_crate_id_hash_conflicts
             at /home/dennis/Projects/rust/rust/compiler/rustc_metadata/src/creader.rs:358:31
   6: rustc_metadata::creader::CrateLoader::register_crate
             at /home/dennis/Projects/rust/rust/compiler/rustc_metadata/src/creader.rs:425:9
   7: rustc_metadata::creader::CrateLoader::maybe_resolve_crate
             at /home/dennis/Projects/rust/rust/compiler/rustc_metadata/src/creader.rs:580:17
   8: rustc_metadata::creader::CrateLoader::resolve_crate
             at /home/dennis/Projects/rust/rust/compiler/rustc_metadata/src/creader.rs:514:9
   9: rustc_metadata::creader::CrateLoader::process_path_extern
             at /home/dennis/Projects/rust/rust/compiler/rustc_metadata/src/creader.rs:1027:20
  10: rustc_resolve::Resolver::extern_prelude_get::{{closure}}
             at /home/dennis/Projects/rust/rust/compiler/rustc_resolve/src/lib.rs:3202:21
      