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
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking std v0.0.0 (/checkout/library/std)
    Checking core v0.0.0 (/checkout/library/core)
error[E0658]: use of unstable library feature 'at_least': new API
     |
     |
1939 |     assert!(vec.iter().at_least(0, |i| i % 2 == 0));
     |
     |
     = help: add `#![feature(at_least)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'at_least': new API
     |
     |
1940 |     assert!(vec.iter().at_least(1, |i| i % 2 == 0));
     |
     |
     = help: add `#![feature(at_least)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'at_least': new API
     |
     |
1941 |     assert!(vec.iter().at_least(2, |i| i % 2 == 0));
     |
     |
     = help: add `#![feature(at_least)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'at_least': new API
     |
     |
1942 |     assert!(!vec.iter().at_least(3, |i| i % 2 == 0));
     |
     |
     = help: add `#![feature(at_least)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'at_least': new API
     |
     |
1943 |     assert!(vec.iter().at_least(0, |&i| i > 100));
     |
     |
     = help: add `#![feature(at_least)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'at_least': new API
     |
     |
1944 |     assert!(vec.iter().at_least(5, |&i| i < 100));
     |
     |
     = help: add `#![feature(at_least)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'at_least': new API
     |
     |
1946 |     assert!(!&vec[..0].iter().at_least(0, |_| panic!()));
     |
     |
     = help: add `#![feature(at_least)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'at_most': new API
     |
     |
1952 |     assert!(vec.iter().at_most(0, |&i| i > 100));
     |
     |
     = help: add `#![feature(at_most)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'at_most': new API
     |
     |
1953 |     assert!(vec.iter().at_most(5, |&i| i < 100));
     |
     |
     = help: add `#![feature(at_most)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'at_most': new API
     |
     |
1954 |     assert!(vec.iter().at_most(3, |i| i % 2 == 0));
     |
     |
     = help: add `#![feature(at_most)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'at_most': new API
     |
     |
1955 |     assert!(vec.iter().at_most(2, |i| i % 2 == 0));
     |
     |
     = help: add `#![feature(at_most)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'at_most': new API
     |
     |
1956 |     assert!(!vec.iter().at_most(1, |i| i % 2 == 0));
     |
     |
     = help: add `#![feature(at_most)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'at_most': new API
     |
     |
1957 |     assert!(!vec.iter().at_most(0, |i| i % 2 == 0));
     |
     |
     = help: add `#![feature(at_most)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'at_most': new API
     |
     |
1959 |     assert!(&vec[..0].iter().at_most(0, |_| panic!()));
     |
     |
     = help: add `#![feature(at_most)]` to the crate attributes to enable
error: aborting due to 14 previous errors

For more information about this error, try `rustc --explain E0658`.
error: could not compile `core`
error: could not compile `core`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--all-targets" "-p" "test" "-p" "core" "-p" "std" "-p" "alloc" "-p" "unwind" "-p" "panic_unwind" "-p" "proc_macro" "-p" "term" "-p" "panic_abort" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:00:48
