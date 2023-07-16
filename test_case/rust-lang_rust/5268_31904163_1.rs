 rust
fn minmax<T: Ord, A: Iterator<T>>(me: &mut A) -> MinMax<T> {
    let (mut min, mut max) = match me.next() {
        None => return NoElements,
        Some(x) => {
            match me.next() {
                None => return OneElement(x),
                Some(y) => if x <= y {(x, y)} else {(y, x)}
            }
        }
    }

    for new in me.iter() {
        if new < min { min = new; }
        else if new > max { max = new; }
    }

    MinMax(min, max)
}
