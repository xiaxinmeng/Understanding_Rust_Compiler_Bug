Rust
if maybe {
    let [_, ..b] = array;
} else {
    let [..b, _, _x] = array;
}
