rust
let opt = Some(Some(1i32));

if (let Some(a) = opt && (let Some(b) = a)) && b == 1 {
    println!("`b` is declared inside but used outside");
}
