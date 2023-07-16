rust
src/libfmt_macros/lib.rs
86:pub enum Alignment {
87-    /// The value will be aligned to the left.
88-    AlignLeft,
89-    /// The value will be aligned to the right.
90-    AlignRight,
91-    /// The value will be aligned in the center.
92-    AlignCenter,
93-    /// The value will take on a default alignment.
94-    AlignUnknown,
95-}

src/libcore/fmt/mod.rs
31:pub enum Alignment {
32-    /// Indication that contents should be left-aligned.
33-    Left,
34-    /// Indication that contents should be right-aligned.
35-    Right,
36-    /// Indication that contents should be center-aligned.
37-    Center,
38-    /// No alignment was requested.
39-    Unknown,
40-}

src/libcore/fmt/rt/v1.rs
35:pub enum Alignment {
36-    /// Indication that contents should be left-aligned.
37-    Left,
38-    /// Indication that contents should be right-aligned.
39-    Right,
40-    /// Indication that contents should be center-aligned.
41-    Center,
42-    /// No alignment was requested.
43-    Unknown,
44-}
