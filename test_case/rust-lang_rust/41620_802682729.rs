rust
let finite = FiniteF64::new(1f64).unwrap();

match finite {
    FiniteF64(1f64) => {
        std::println!("A")
    }
    FiniteF64(2f64) => {
        std::println!("B")
    }
    _ => {
        std::println!("C")
    }
}
