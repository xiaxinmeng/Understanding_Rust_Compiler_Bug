
#![feature(non_ascii_idents)]
fn main() {
    fn simulate() -> Vec<Coord> { vec![] }
    struct Coord { t: f64, θ: f64 }
    let data: Vec<Coord> = simulate();
    let t = data.iter().map(|d| d.t);
    let θ = data.iter().map(|d| d.θ);
}
