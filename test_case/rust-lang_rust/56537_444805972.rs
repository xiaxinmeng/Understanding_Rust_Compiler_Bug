rust
﻿﻿fn print_first(list: Vec<String>) {
    let x: &str = list
        .first()
        .map(|s| -> &str { &s[..] })
        .unwrap_or("");
    println!("First element is {}", x);
}
