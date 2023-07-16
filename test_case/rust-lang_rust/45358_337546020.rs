rust
/// SomeType is u32 when feature is on, and u64 when feature is off.
#[cfg(feature = "feature")]
pub type SomeType = u32;
/// SomeType is u32 when feature is on, and u64 when feature is off.
#[cfg(not(feature = "feature"))]
pub type SomeType = u64;

/// A struct
pub struct Struct {
    /// A field
    pub field: SomeType,
}
