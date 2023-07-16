rust
error: reachable patterns not covered of non exhaustive enum
  --> /home/devinr/aprog/rust/__forks__/rust/src/test/ui/rfc-2008-non-exhaustive/reachable-patterns.rs:116:9
   |
LL |         _ => {}
   |         ^ patterns `HostUnreachable`, `NetworkUnreachable`, `NetworkDown` and 18 more not covered
   |
note: the lint level is defined here
  --> /home/devinr/aprog/rust/__forks__/rust/src/test/ui/rfc-2008-non-exhaustive/reachable-patterns.rs:93:12
   |
LL |     #[deny(non_exhaustive_reachable_patterns)]
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = help: ensure that all possible cases are being handled by adding the suggested match arms
   = note: the matched value is of type `ErrorKind` and the wildcard has the `non_exhaustive_reachable_patterns` attribute

error: aborting due to 6 previous errors
