rust
enum DurationConversionError {
    NotFinite, // note: maybe distinguish between Inf/-Inf and NaN floating point values?
    Negative,
    Overflow
}
