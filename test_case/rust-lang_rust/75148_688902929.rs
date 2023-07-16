plain
[RUSTC-TIMING] core test:false 24.710
[RUSTC-TIMING] addr2line test:false 0.453
[RUSTC-TIMING] gimli test:false 5.754
[RUSTC-TIMING] object test:false 11.427
error[E0425]: cannot find function `peer_cred` in module `ucred`
    |
    |
428 |         ucred::peer_cred(self)
    |                ^^^^^^^^^ not found in `ucred`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0425`.
[RUSTC-TIMING] std test:false 2.544
---
== clock drift check ==
  local time: Tue Sep  8 14:10:56 UTC 2020
  network time: Tue, 08 Sep 2020 14:10:56 GMT
== end clock drift check ==
##[error]Process completed with exit code 1.
Terminate orphan process: pid (6415) (node)
Terminate orphan process: pid (6424) (python)
