 rust
if strs[0].ends_with(",)") {
    strs[0][1 .. strs[0].len() - 2] // Remove '(' and ',)'
} else {
    strs[0]
}
