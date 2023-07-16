rust
Testing std stage1 (i686-pc-windows-msvc -> i686-pc-windows-msvc)
   Compiling std v0.0.0 (C:\projects\rust\src\libstd)
[RUSTC-TIMING] run_time_detect test:true 1.188
[RUSTC-TIMING] env test:true 1.890
error[E0433]: failed to resolve: use of undeclared type or module `os`
    --> src\libstd\net\tcp.rs:1575:36
     |
1575 |         fn render_inner(addr: &dyn os::windows::io::AsRawSocket) -> impl fmt::Debug {
     |                                    ^^ use of undeclared type or module `os`
error: aborting due to previous error
For more information about this error, try `rustc --explain E0433`.
