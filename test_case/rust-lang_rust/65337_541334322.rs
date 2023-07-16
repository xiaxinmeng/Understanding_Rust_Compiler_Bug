plain
2019-10-12T15:18:56.5517007Z    Compiling test v0.0.0 (/checkout/src/libtest)
2019-10-12T15:19:10.6588421Z [RUSTC-TIMING] test test:false 14.103
2019-10-12T15:19:10.6606017Z     Finished release [optimized + debuginfo] target(s) in 1m 30s
2019-10-12T15:19:10.6743818Z Copying stage0 std from stage0 (x86_64-unknown-linux-musl -> x86_64-unknown-linux-musl / x86_64-unknown-linux-musl)
2019-10-12T15:19:10.6756233Z thread 'main' panicked at 'src.symlink_metadata() failed with No such file or directory (os error 2)', src/bootstrap/lib.rs:1148:24
2019-10-12T15:19:10.7001045Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --build x86_64-unknown-linux-musl
2019-10-12T15:19:10.7001804Z Build completed unsuccessfully in 0:01:32
2019-10-12T15:19:10.7055085Z == clock drift check ==
2019-10-12T15:19:10.7073596Z   local time: Sat Oct 12 15:19:10 UTC 2019
2019-10-12T15:19:10.7073596Z   local time: Sat Oct 12 15:19:10 UTC 2019
2019-10-12T15:19:10.8723848Z   network time: Sat, 12 Oct 2019 15:19:10 GMT
2019-10-12T15:19:10.8724299Z == end clock drift check ==
2019-10-12T15:19:11.6303780Z ##[error]Bash exited with code '1'.
2019-10-12T15:19:11.6355761Z ##[section]Starting: Upload CPU usage statistics
2019-10-12T15:19:11.6373706Z ==============================================================================
2019-10-12T15:19:11.6373824Z Task         : Bash
2019-10-12T15:19:11.6373902Z Description  : Run a Bash script on macOS, Linux, or Windows
