rust
println!("{}", "böb".chars().all(char::is_alphanumeric));    // true
println!("{}", "böb".chars().all(char::is_alphanumeric));    // false
