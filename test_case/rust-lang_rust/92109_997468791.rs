
rustc +nightly -Zinstrument-mcount -C passes="ee-instrument post-inline-ee-instrument" main.rs
