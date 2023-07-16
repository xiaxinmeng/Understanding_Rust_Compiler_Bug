
[00:53:11] error[E0433]: failed to resolve. Use of undeclared type or module `io`
[00:53:11]    --> libstd/sys/redox/net/mod.rs:103:31
[00:53:11]     |
[00:53:11] 103 | pub fn on_resolver_failure(e: io::Error) -> io::Error { e }
[00:53:11]     |                               ^^ Use of undeclared type or module `io`
[00:53:11] 
[00:53:11] error[E0433]: failed to resolve. Use of undeclared type or module `io`
[00:53:11]    --> libstd/sys/redox/net/mod.rs:103:45
[00:53:11]     |
[00:53:11] 103 | pub fn on_resolver_failure(e: io::Error) -> io::Error { e }
[00:53:11]     |                                             ^^ Use of undeclared type or module `io`
[00:53:11] 
[00:53:16] error: aborting due to 2 previous errors
[00:53:16] 
[00:53:16] error: Could not compile `std`.
