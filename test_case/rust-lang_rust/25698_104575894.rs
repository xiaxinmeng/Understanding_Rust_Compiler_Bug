 rust
let letters = ["alpha", "beta", "gamma"];
assert_eq!(letters.iter()
                  .flat_map(|s| s.chars())
                  .collect::<String>(),
           "alphabetagamma");
