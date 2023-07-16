plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 6a705566166debf5eff88c57140df607fa409aaa and 81c66b1cc939fc113cab86bdcb457f27c4953e88
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
   Compiling crypto-hash v0.3.4
   Compiling rustc-workspace-hack v1.0.0 (/checkout/src/tools/rustc-workspace-hack)
   Compiling cargo-util v0.1.2 (/checkout/src/tools/cargo/crates/cargo-util)
   Compiling git2-curl v0.14.1
warning: use of deprecated struct `clap::App`: Replaced with `Command`
   |
   |
25 | pub type App = clap::App<'static>;
   |
   = note: `#[warn(deprecated)]` on by default


warning: use of deprecated unit variant `clap::AppSettings::DontCollapseArgsInUsage`: Replaced with `Command::dont_collapse_args_in_usage` and `Command::is_dont_collapse_args_in_usage_set`
    |
    |
284 |     App::new(name).setting(AppSettings::DeriveDisplayOrder | AppSettings::DontCollapseArgsInUsage)

warning: `cargo` (lib) generated 2 warnings
    Finished release [optimized] target(s) in 3m 36s
    Finished release [optimized] target(s) in 3m 36s
 Downloading crates ...
  Downloaded lsp-codec v0.3.0
  Downloaded difference v2.0.0
   Compiling difference v2.0.0
   Compiling rls v1.41.0 (/checkout/src/tools/rls)
   Compiling lsp-codec v0.3.0
   Compiling rls-rustc v0.6.0 (/checkout/src/tools/rls/rls-rustc)
warning: use of deprecated struct `clap::App`: Replaced with `Command`
   |
   |
25 | pub type App = clap::App<'static>;
   |
   = note: `#[warn(deprecated)]` on by default


warning: use of deprecated unit variant `clap::AppSettings::DontCollapseArgsInUsage`: Replaced with `Command::dont_collapse_args_in_usage` and `Command::is_dont_collapse_args_in_usage_set`
    |
    |
284 |     App::new(name).setting(AppSettings::DeriveDisplayOrder | AppSettings::DontCollapseArgsInUsage)

warning: `cargo` (lib) generated 2 warnings
warning: field is never read: `clippy_preference`
   --> src/tools/rls/rls/src/build/rustc.rs:215:5
---
Verifying status of rls...
Verifying status of miri...
This PR updated 'src/tools/miri', verifying if status is 'test-pass'...

We detected that this PR updated 'miri', but its tests failed.

If you do intend to update 'miri', please check the error messages above and
commit another update.

If you do NOT intend to update 'miri', please ensure you did not accidentally
change the submodule at 'src/tools/miri'. You may ask your reviewer for the
