rs
let s: String = "hola".to_owned();

let outer = new_func(move || {
    let inner = move || println!("{s}");
    inner()
});
