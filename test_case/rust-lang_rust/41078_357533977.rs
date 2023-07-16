rust
let s = "hello";
let re = regex::Regex::new("[aeiou]").unwrap();
let after = re.replace_all(s, |c| "xxx".to_string());
println!("{} -> {}", s, after);
