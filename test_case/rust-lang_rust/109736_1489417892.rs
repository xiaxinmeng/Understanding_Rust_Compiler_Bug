rust
let env = "hello".to_string();

let ok1 = Lazy::new(|| env);
let ok2: Lazy<String, _> = Lazy::new(|| env);

let err: Lazy<String> = Lazy::new(|| env);
// ^ The confusing case. The problem here is that type of `F` isn't inferred, 
/// but is taken from the default
