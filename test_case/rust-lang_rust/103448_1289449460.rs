plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between c07a6a9c0c6c571d535de74d7ec28469fba46e37 and 0a153e3b1b51ad3bcbb4d00e7fbc0fe2932bcc7b
Tool subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
   Compiling shell-escape v0.1.5
   Compiling parking_lot v0.11.2
   Compiling measureme v10.1.0
   Compiling libffi v3.0.1
error[E0599]: no method named `no_bound_vars` found for struct `rustc_middle::ty::Ty` in the current scope
    |
    |
282 |             let main_ret_ty = main_ret_ty.no_bound_vars().unwrap();

For more information about this error, try `rustc --explain E0599`.
error: could not compile `miri` due to previous error
error: could not compile `miri` due to previous error
thread 'main' panicked at 'in-tree tool', test.rs:489:14
Build completed unsuccessfully in 0:00:13
