rust
pub enum MirPhase {
    Built,              // MirPhase::Built, 
    Const,              //
    ConstsPromoted,     // AnalysisPhase::Initial
    Derefered,          // AnalysisPhase::PostCleanup
    DropsLowered,       //
    GeneratorsLowered,  // (`StateTransform` run before `Deaggregator` in the change)
    Deaggregated,       // RuntimePhase::Initial
                        // RuntimePhase::PostCleanup
    Optimized,          // RuntimePhase::Optimized
}
