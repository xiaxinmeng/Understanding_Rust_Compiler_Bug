 rust
//  Increment elements in row `i` by 1.0
array.row_mut(i) += 1.0;
// scale the 4 x 4 top left quadrant by alpha
array.slice_mut(s![..4, ..4]) *= alpha; 
