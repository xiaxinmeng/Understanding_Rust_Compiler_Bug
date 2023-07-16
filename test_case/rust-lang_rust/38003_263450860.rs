rust
/// An abstraction over a hashtable's load factor which allows the user to choose which aspect they
/// would prefer to optimize, possibly at a detriment to other metrics.
pub enum LoadFactor {
    /// Use a load factor that produces the fastest lookup times without seriously degrading other aspects.
    OptimizeLookup,
    /// Use a load factor that produces the fastest insertion times without seriously degrading other aspects.
    OptimizeInsert,
    /// Use a load factor that produces the lowest memory usage without seriously degrading other aspects.
    SaveMemory,
    /// Use a load-factor with good all-round performance.
    Balanced,
    /// A custom load factor whose performance may be affected by changes to the implementation.
    Custom(f64),
}
