rust
#[repr(transparent, align = "128")]
struct BogusAlign(f64);

#[repr(transparent, packed)]
struct BogusPacked(f64);
