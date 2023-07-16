rust
fn discovered_universal_model_of_computation(name: &str) {
    if let "Alan" | "Alonzo" | "Kurt" = name {
        println!("Yes, {} did discover a turing equivalent model of computation.", name);
    } else {
        println!("Nope, but {} was pretty cool nonetheless.", name);
    }
}

fn main() {
    for name in &["Alan", "Alonzo", "Kurt", "Voevodsky"] {
        discovered_universal_model_of_computation(name);
    }
}
