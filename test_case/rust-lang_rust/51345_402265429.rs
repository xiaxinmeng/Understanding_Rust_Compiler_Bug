rust
enum TwoPhaseActivation {
    NotTwoPhase, // what is now `None`
    NotActivated,
    ActivatedAt(Location), // roughly what is now `Some`
}
