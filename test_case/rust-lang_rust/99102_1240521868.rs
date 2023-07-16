rust
enum MirChange {
    PhaseChange(MirPhase),
    Pass(&dyn MirPass),
}
