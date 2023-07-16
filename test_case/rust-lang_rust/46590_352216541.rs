rust
pub(crate) struct Cause {
    /// Length of the outlives chain. This is intentionally first so that we
    /// prefer causes that are more directly related (fewer outlives links).
    outlives: u32,

    /// The "root cause" -- basically, what is live?
    root_cause: RootCause,
}

// (this is the old cause enum, minus the `Outlives` variant)
pub(crate) enum RootCause {
    /// point inserted because Local was live at the given Location
    LiveVar(Local, Location),

    /// point inserted because Local was dropped at the given Location
    DropVar(Local, Location),

    /// point inserted because the type was live at the given Location,
    /// but not as part of some local variable
    LiveOther(Location),

    /// part of the initial set of values for a universally quantified region
    UniversalRegion(RegionVid),
}
