rs
let s: String = "hola".to_owned();

let outer2 = move |val| {
    let inner2 = move || println!("{val} {s}");
    inner2()
};
