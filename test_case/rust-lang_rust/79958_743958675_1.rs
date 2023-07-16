text
===================================================================================================

Failures due to:
    error[E0463]: can't find crate for `profiler_builtins`
The tests' `compiler-flags` for these tests specify targets, and
the targets' rustc build `config.toml`s must not have included `profiler = true`

src/test/ui/asm/bad-arch.rs
    # --target sparc-unknown-linux-gnu
src/test/ui/issues/issue-50993.rs
    # --target thumbv7em-none-eabihf
src/test/ui/sanitize/unsupported-target.rs
    # --target i686-unknown-linux-gnu
