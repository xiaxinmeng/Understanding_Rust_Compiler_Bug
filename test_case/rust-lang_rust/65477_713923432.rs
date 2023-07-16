rust
match x { // single range pattern
    0 ..= 125 => {}
    125 ..= 255 => {} // overlap detected
}
match x { // anything else
    (0 ..= 125, true) => {}
    (125 ..= 255, true) => {} // overlap not detected
}
