rust
const LEADING: &str = "enum __StaticMap__ {\n    A =\n        static_map!(@ zero";
const TRAILING: &str = "),\n}";

fn trim(input: &str) -> &str {
    assert!(input.starts_with(LEADING));
    assert!(input.ends_with(TRAILING));

    let (_, input) = input.split_at(LEADING.len());
    let (input, _) = input.split_at(input.len() - TRAILING.len());
    input.trim()
}
