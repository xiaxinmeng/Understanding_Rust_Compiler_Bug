rust
// Try to make the position of the pivot after partitioning
// is under 12.5% (1/8th) percentile(or over 87.5% percentile).
let unbalanced_pivot_idx = len / 8 - 5;
input.select_nth_unstable(unbalanced_pivot_idx);
input.swap(unbalanced_pivot_idx, len / 2);
