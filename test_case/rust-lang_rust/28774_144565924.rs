 rust
if is_first_row_collected {
    assert!(inner_cols == cols);
} else {
    is_first_row_collected = true;
    cols = inner_cols;
}
rows += 1;

let m = MatrixXf{
    values: tmp_vec, 
    rows: rows,
    cols: cols
};
