
error[E0433]: failed to resolve. Could not find `unreachable` in `$crate`
  --> examples/csp.rs:40:5
   |
40 |     select! { default => {} }
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^ Could not find `unreachable` in `$crate`
   |
   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
