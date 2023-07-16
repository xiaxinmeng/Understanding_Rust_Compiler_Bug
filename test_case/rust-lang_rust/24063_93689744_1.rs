 rust
// no ICE; I would have thought "identical" to the above...
for index in 0..count_map.len() {
    while let Some((time, delta)) = count_map[index].pop() {
        self.frontier[index].update(&time, delta);
    }
}
