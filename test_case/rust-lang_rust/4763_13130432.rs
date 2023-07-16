
fn next<T>(iter: &mut Iterator/&a<T>) -> bool {
    iter.index += 1; // Relies on rollover
    iter.index < iter.vector.len()
}
