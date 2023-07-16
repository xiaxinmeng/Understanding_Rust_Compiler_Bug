
<core macros>:2:4: 2:12 error: cannot apply unary operator `!` to type `&sync::once::OnceState`
<core macros>:2 if ! $ cond {
                   ^~~~~~~~
../src/libstd/sync/once.rs:443:13: 443:30 note: in this expansion of assert! (defined in <core macros>)
<core macros>:2:4: 2:12 error: cannot apply unary operator `!` to type `&sync::once::OnceState`
<core macros>:2 if ! $ cond {
                   ^~~~~~~~
../src/libstd/sync/once.rs:466:17: 466:28 note: in this expansion of assert! (defined in <core macros>)
error: aborting due to 2 previous errors
make: *** [x86_64-unknown-linux-gnu/stage2/test/stdtest-x86_64-unknown-linux-gnu] Error 101
