rust
enum StrandFail {
    NoSolution,
    QuantumExceeded,
    Cycle(Strand, Minimums),
}
