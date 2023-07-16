
help: Your first argument should be a String, not &str. Try using `to_string()`
|            println!("{}", expr(34: "Hello").to_string() + " World");
