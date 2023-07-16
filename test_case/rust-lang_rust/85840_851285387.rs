
use std::fs;
fn repro() {
    fs::read("/absolute/path.txt").unwrap();
}
fn main() {
    repro();
}
