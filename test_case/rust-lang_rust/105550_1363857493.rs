plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
configure: 
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
/checkout/src/ci/run.sh: line 195: [: =: unary operator expected
/checkout/src/ci/run.sh: line 199: [: =: unary operator expected

###############################                                           43.9%
######################################################################## 100.0%
extracting /checkout/obj/build/cache/2022-11-01/rust-std-beta-x86_64-unknown-linux-gnu.tar.xz
---
    Checking rustc_parse v0.0.0 (/checkout/compiler/rustc_parse)
error[E0091]: type parameter `D` is unused
   --> compiler/rustc_query_system/src/query/job.rs:290:13
    |
290 | type Waiter<D> = (QueryJobId, usize);
    |             ^ unused type parameter
For more information about this error, try `rustc --explain E0091`.
error: could not compile `rustc_query_system` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_query_system` due to previous error
