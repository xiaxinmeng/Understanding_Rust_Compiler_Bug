plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
    Checking rand v0.7.3
    Checking std v0.0.0 (/checkout/library/std)
    Checking core v0.0.0 (/checkout/library/core)
    Checking alloc v0.0.0 (/checkout/library/alloc)
error[E0425]: cannot find value `max` in this scope
    |
    |
365 |     assert_eq!(max, half_max.saturating_mul(four));
    |
help: consider importing one of these items
    |
1   | use core::cmp::max;
---
1   | use crate::num::f64::max;
    |
      and 1 other candidate

error[E0425]: cannot find value `max` in this scope
    |
    |
383 |     assert_eq!(max, half_max.saturating_pow(3));
    |
help: consider importing one of these items
    |
1   | use core::cmp::max;
---
1   | use crate::num::f64::max;
    |
      and 1 other candidate

error[E0658]: use of unstable library feature 'nonzero_add'
    |
    |
337 |     assert_eq!(Some(two), one.checked_add(1));
    |
    |
    = help: add `#![feature(nonzero_add)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'nonzero_add'
    |
    |
338 |     assert_eq!(None, max.checked_add(1));
    |
    |
    = help: add `#![feature(nonzero_add)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'nonzero_add'
    |
    |
346 |     assert_eq!(two, one.saturating_add(1));
    |
    |
    = help: add `#![feature(nonzero_add)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'nonzero_add'
    |
    |
347 |     assert_eq!(max, max.saturating_add(1));
    |
    |
    = help: add `#![feature(nonzero_add)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'nonzero_mul'
    |
    |
355 |     assert_eq!(Some(max), half_max.checked_mul(two));
    |
    |
    = help: add `#![feature(nonzero_mul)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'nonzero_mul'
    |
    |
356 |     assert_eq!(None, half_max.checked_mul(max));
    |
    |
    = help: add `#![feature(nonzero_mul)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'nonzero_mul'
    |
    |
364 |     assert_eq!(four, two.saturating_mul(two));
    |
    |
    = help: add `#![feature(nonzero_mul)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'nonzero_mul'
    |
    |
365 |     assert_eq!(max, half_max.saturating_mul(four));
    |
    |
    = help: add `#![feature(nonzero_mul)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'nonzero_pow'
    |
    |
373 |     assert_eq!(Some(twenty_seven), three.checked_pow(3));
    |
    |
    = help: add `#![feature(nonzero_pow)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'nonzero_pow'
    |
    |
374 |     assert_eq!(None, half_max.checked_pow(3));
    |
    |
    = help: add `#![feature(nonzero_pow)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'nonzero_pow'
    |
    |
382 |     assert_eq!(twenty_seven, three.saturating_pow(3));
    |
    |
    = help: add `#![feature(nonzero_pow)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'nonzero_pow'
    |
    |
383 |     assert_eq!(max, half_max.saturating_pow(3));
    |
    |
    = help: add `#![feature(nonzero_pow)]` to the crate attributes to enable
error: aborting due to 14 previous errors

Some errors have detailed explanations: E0425, E0658.
For more information about an error, try `rustc --explain E0425`.
For more information about an error, try `rustc --explain E0425`.
error: could not compile `core`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--all-targets" "-p" "test" "-p" "panic_unwind" "-p" "unwind" "-p" "alloc" "-p" "core" "-p" "term" "-p" "std" "-p" "panic_abort" "-p" "proc_macro" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:00:56
