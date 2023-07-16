
let mut name = Some(name);
...
for parser.each_token |token| {
    match token {
        Delim('!') => {
            let name = name.swap_unwrap(); // or `Option::swap_unwrap(&mut name);`
            ...
