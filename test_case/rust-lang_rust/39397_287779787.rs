rust
let opt: &Option<String>;
// ...
let result: &str = opt // &Option<String>
    .as_ref() // Option<&String>
    .map(|x| x.as_ref()) // Option<&str> -- this step could be omitted
    .unwrap_or(".. nothing there"); // &str
