rust
#[proc_macro]
fn item_macro(input) {
    "fn item() {
        ${input}
        let y = x; // OK    
    }"
}

item_macro!(let x = 10);
