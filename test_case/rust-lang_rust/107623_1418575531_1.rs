rs
let s: String = "hola".to_owned();

let outer = move |cond| {
    if cond {
        let inner = move || println!("{s}");
        inner()
    } else {
        let inner = move || println!("{s}");
        inner()
    }
};
