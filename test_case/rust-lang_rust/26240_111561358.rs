 console
$ DYLD_LIBRARY_PATH=~/Applications/rust-1.0.0/lib rustc -o error-index-generator src/error-index-generator/main.rs 
src/error-index-generator/main.rs:23:37: 23:53 error: unresolved import `syntax::diagnostics::metadata::get_metadata_dir`. Could not find `metadata` in `syntax::diagnostics`
src/error-index-generator/main.rs:23 use syntax::diagnostics::metadata::{get_metadata_dir, ErrorMetadataMap};
                                                                         ^~~~~~~~~~~~~~~~
src/error-index-generator/main.rs:23:55: 23:71 error: unresolved import `syntax::diagnostics::metadata::ErrorMetadataMap`. Could not find `metadata` in `syntax::diagnostics`
src/error-index-generator/main.rs:23 use syntax::diagnostics::metadata::{get_metadata_dir, ErrorMetadataMap};
                                                                                           ^~~~~~~~~~~~~~~~
error: aborting due to 2 previous errors
