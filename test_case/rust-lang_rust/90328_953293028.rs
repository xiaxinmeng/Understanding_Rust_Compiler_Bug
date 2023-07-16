plain
   Compiling alloc v0.0.0 (/checkout/library/alloc)
   Compiling cfg-if v0.1.10
   Compiling adler v0.2.3
   Compiling rustc-demangle v0.1.21
error[E0599]: no function or associated item named `downcast` found for trait object `dyn core::error::Error` in the current scope
     |
     |
2101 |         <dyn Error>::downcast(err).map_err(|s| unsafe {
     |                      ^^^^^^^^ function or associated item not found in `dyn core::error::Error`

error[E0599]: no function or associated item named `downcast` found for trait object `dyn core::error::Error` in the current scope
     |
     |
2115 |         <dyn Error>::downcast(err).map_err(|s| unsafe {
     |                      ^^^^^^^^ function or associated item not found in `dyn core::error::Error`
For more information about this error, try `rustc --explain E0599`.
error: could not compile `alloc` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: build failed
