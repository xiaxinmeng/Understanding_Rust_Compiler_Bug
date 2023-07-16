plain
[command]/usr/bin/git submodule foreach --recursive git config --local --name-only --get-regexp 'http\.https\:\/\/github\.com\/\.extraheader' && git config --local --unset-all 'http.https://github.com/.extraheader' || :
[command]/usr/bin/git config --local http.https://github.com/.extraheader AUTHORIZATION: basic ***
##[endgroup]
##[group]Fetching the repository
[command]/usr/bin/git -c protocol.version=2 fetch --no-tags --prune --progress --no-recurse-submodules --depth=2 origin +73d05dfcf8c2036697eacb0514a57f49bb3c1491:refs/remotes/pull/80085/merge
---
   Compiling libc v0.2.79
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.36
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0541]: unknown meta item 'since'
     |
     |
1808 |     #[unstable(feature = "iter_is_partitioned", since = "1.49.0")]
     |                                                 ^^^^^^^^^^^^^^^^ expected one of `feature`, `reason`, `issue`, `soft`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0541`.
error: could not compile `core`
