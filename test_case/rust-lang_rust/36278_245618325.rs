 rust
let fields = variant.offset_after_field.len();
if fields >= 2 {
    variant.offset_after_field[fields - 2].bytes()
} else {
    0
}
