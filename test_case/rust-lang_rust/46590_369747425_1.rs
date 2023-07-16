rust
impl RegionInferenceContext {
    crate fn compute_causal_info(&self) -> RegionCausalInfo {
        // same thing as `propagate_constraints` today:
        // - clone the liveness values (keeping the causal information this time!)
        // - propagate the constraints on those values
    }
}
