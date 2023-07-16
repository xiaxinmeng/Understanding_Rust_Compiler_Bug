rust
match (foo > 0, bar > 0) {
    (true, true) | (false, false) => max,
    (false, true) | (true, false) => min,
    _ => zero,
}
