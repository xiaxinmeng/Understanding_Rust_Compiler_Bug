rs
struct WorldCup {
   r#match: String,
}

fn main() {
    let wc = WorldCup { r#match: "19 July 1930 Argentina 6-3 Mexico".to_string() };
    println!("{}", wc.r#match);
}
