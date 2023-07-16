rust
/// The various "modes" of invoking Cargo.
///
/// These entries currently correspond to the various output directories of the
/// build system, with each mod generating output in a different directory.
#[derive(Clone, Copy)]
pub enum Mode {
    /// This cargo is going to build the standard library, placing output in the
    /// "stageN-std" directory.
    Libstd,

    /// This cargo is going to build libtest, placing output in the
    /// "stageN-test" directory.
    Libtest,

    /// This cargo is going to build librustc and compiler libraries, placing
    /// output in the "stageN-rustc" directory.
    Librustc,

    /// This cargo is going to some build tool, placing output in the
    /// "stageN-tools" directory.
    Tool,
}
