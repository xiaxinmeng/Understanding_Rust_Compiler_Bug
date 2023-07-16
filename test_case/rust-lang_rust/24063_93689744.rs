 rust
// ICEs with assertion failed: bound_list_is_sorted(&bounds.projection_bounds)
for index in 0..count_map.len() {
    while let Some((ref time, delta)) = count_map[index].pop() {
        self.frontier[index].update(time, delta);
    }
}
