 rust
#[derive(Copy)] struct Struct { .. }
fn constructor(r: Range<i32>) -> Struct {
    Struct { start: r.start, end: r.end, .. }
}
