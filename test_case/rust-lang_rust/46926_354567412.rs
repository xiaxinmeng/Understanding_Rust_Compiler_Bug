rust
if other > self || other.is_nan() { self } else { other } // 16 instructions (min)
if other.is_nan() || other > self { self } else { other } // 11 instructions (min)
