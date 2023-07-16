rust
/// The styles of printing that we can print
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PrintFmt {
    /// Prints a terser backtrace which ideally only contains relevant information
    Short,
    /// Prints a backtrace that contains all possible information
    Full,
    #[doc(hidden)]
    __Nonexhaustive,
}
