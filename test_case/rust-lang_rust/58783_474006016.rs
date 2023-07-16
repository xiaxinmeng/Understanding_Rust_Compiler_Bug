
thread 'rustc' panicked at 'attempt to create unaligned or null slice', src/libcore/slice/mod.rs:4925:5acro2, chalk-engine, rustc_version, min...
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.ngine, serialize, rustc_apfloat, num_cpus, jobserver, m...
stack backtrace:
   9: core::panicking::panic
             at src/libcore/panicking.rs:49
  10: rustc_codegen_llvm::metadata::search_meta_section
             at ./src/libcore/macros.rs:10
             at src/librustc_codegen_llvm/metadata.rs:72
  11: <rustc_codegen_llvm::metadata::LlvmMetadataLoader as rustc::middle::cstore::MetadataLoader>::get_dylib_metadata
             at src/librustc_codegen_llvm/metadata.rs:57
             at ./src/librustc_data_structures/owning_ref/mod.rs:412
             at src/librustc_codegen_llvm/metadata.rs:57
  12: rustc_metadata::locator::get_metadata_section
             at src/librustc_metadata/locator.rs:888
             at src/librustc_metadata/locator.rs:857
  13: rustc_metadata::locator::Context::extract_one
             at src/librustc_metadata/locator.rs:620
  14: rustc_metadata::locator::Context::find_commandline_library
             at src/librustc_metadata/locator.rs:834
  15: rustc_metadata::locator::Context::find_library_crate
             at src/librustc_metadata/locator.rs:448
  16: rustc_metadata::locator::Context::maybe_load_library_crate
             at src/librustc_metadata/locator.rs:320
  17: rustc_metadata::creader::CrateLoader::load
             at src/librustc_metadata/creader.rs:400
  18: rustc_metadata::creader::CrateLoader::resolve_crate
             at src/librustc_metadata/creader.rs:375
  19: rustc_metadata::creader::CrateLoader::process_extern_crate
             at src/librustc_metadata/creader.rs:1065
...
