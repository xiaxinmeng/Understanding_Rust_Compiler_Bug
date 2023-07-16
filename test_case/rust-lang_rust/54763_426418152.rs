plain
[00:18:24]    Compiling rustc_passes v0.0.0 (/checkout/src/librustc_passes)
[00:18:41]    Compiling rustc_plugin v0.0.0 (/checkout/src/librustc_plugin)
[00:18:41]    Compiling rustc_resolve v0.0.0 (/checkout/src/librustc_resolve)
[00:18:42]    Compiling rustc_save_analysis v0.0.0 (/checkout/src/librustc_save_analysis)
[00:18:44] error[E0609]: no field `extern_prelude` on type `&mut resolve_imports::ImportResolver<'a, 'b, 'c>`
[00:18:44]    --> librustc_resolve/error_reporting.rs:135:41
[00:18:44]     |
[00:18:44] 135 |         let external_crate_names = self.extern_prelude.clone();
[00:18:44] 
[00:18:44] error: aborting due to previous error
[00:18:44] 
[00:18:44] For more information about this error, try `rustc --explain E0609`.
