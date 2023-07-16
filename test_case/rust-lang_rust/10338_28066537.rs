 rust
// Actual number in a string, to avoid tokenizing issues
base!(12, "1ab")
base!(1, "111111")
// Option to separate individual digits with a comma
// (similar to how sexagesimal digits were written by the Babylonians)
// thus avoiding an arbitrary maximum base. Individual digits written in
// the natively supported bases (2, 8, 10, 16)
base!(64, "10,32,17")
// Possible idea of where to stick the type ascription
base!(64, "32", u32)
// Also could work with floating point literals
// (although those actually have a clear use case
// and probably should be added in a non-syntax extension manner)
base!(16, "0x1.fffffffffffffp+1023", f32)
