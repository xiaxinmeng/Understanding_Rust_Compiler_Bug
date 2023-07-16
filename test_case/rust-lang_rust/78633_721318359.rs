rust
fn main() {
    let arena = 0;
    let mut track = Vec::new();
    (move || {
        //let mut track = track;
        track.push(&arena)
    })();
}
