rust
#[derive(Copy, Clone, Debug)]
enum ModuleOrUniformRoot<'a> {
    /// Regular module.
    Module(Module<'a>),

    /// The `{{root}}` (`CrateRoot` aka "global") / `extern` initial segment
    /// in which external crates resolve, and also `crate` (only in `{{root}}`,
    /// but *not* `extern`), in the Rust 2018 edition.
    UniformRoot(Name),
}
