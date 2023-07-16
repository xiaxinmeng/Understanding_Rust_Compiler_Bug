rust
let vec = vec![Some(1), Some(2), None, Some(3)];
for x in vec.into_iter().map_while(|x| x) {
    // the loop body is executed twice
}
