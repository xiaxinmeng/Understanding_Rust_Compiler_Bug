
// I want the guard to ONLY apply to the latter case
0..=9 | (10 if inner <= 7) => (outer * 12) + inner,

// I want the guard to apply to BOTH cases
(0..=9 | 10) if inner <= 7 => (outer * 12) + inner,
