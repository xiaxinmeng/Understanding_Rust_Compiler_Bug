rust
enum StrandFail<T> {
    Success(T),
    NoSolution,
    QuantumExceeded,
    Cycle(Strand, Minimums),
}
