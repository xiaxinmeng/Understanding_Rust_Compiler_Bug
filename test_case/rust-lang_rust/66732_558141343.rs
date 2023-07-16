plain
2019-11-25T12:49:48.2733974Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-11-25T12:49:48.4584962Z error[E0405]: cannot find trait `HashStableContext` in module `crate`
2019-11-25T12:49:48.4585637Z     --> src/librustc_errors/lib.rs:1000:62
2019-11-25T12:49:48.4586063Z      |
2019-11-25T12:49:48.4586432Z 1000 |   #[derive(Clone, Copy, Debug, RustcEncodable, RustcDecodable, HashStable_Generic)]
2019-11-25T12:49:48.4587340Z      |                                                                |
2019-11-25T12:49:48.4587763Z      |                                                                not found in `crate`
2019-11-25T12:49:48.4588128Z      |                                                                in this macro invocation
2019-11-25T12:49:48.4588357Z      |
---
2019-11-25T12:49:51.2701233Z   local time: Mon Nov 25 12:49:51 UTC 2019
2019-11-25T12:49:51.4209914Z   network time: Mon, 25 Nov 2019 12:49:51 GMT
2019-11-25T12:49:51.4220111Z == end clock drift check ==
2019-11-25T12:49:52.7816662Z 
2019-11-25T12:49:52.7930039Z ##[error]Bash exited with code '1'.
2019-11-25T12:49:52.7963550Z ##[section]Starting: Checkout
2019-11-25T12:49:52.7965540Z ==============================================================================
2019-11-25T12:49:52.7966000Z Task         : Get sources
2019-11-25T12:49:52.7966269Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
