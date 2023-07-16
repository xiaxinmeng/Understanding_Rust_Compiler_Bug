rust
if len1 + len2 < 4096 {
    2 * (len1 + len2) < len2 * log2_fast(len1)
} else {
    2 * (len1 + len2) < len2 * 11
}
