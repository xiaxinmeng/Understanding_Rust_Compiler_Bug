
error[E0308]: mismatched types
   --> src/main.rs:17:15
    |
 17 |         func: &foo, //~ ERROR mismatched types
    |               ^^^^ expected fn pointer, found fn item
    |
    = note: expected reference `&fn [1]() -> Option<isize>`
               found reference `&fn [2]() -> Option<isize>`
   = note: fn item [1]: `a::foo`
           fn item [2]: `b::foo`
