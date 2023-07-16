rust
#[repr(transparent, align = "128")]
struct OverAligned(f64); // Behaves as a bare f64 with 128 bits alignment.

#[repr(C, transparent)]
struct BogusRepr(f64); // Nonsensical, repr cannot be C and transparent.
