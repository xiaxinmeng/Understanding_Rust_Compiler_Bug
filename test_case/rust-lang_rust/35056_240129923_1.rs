 rust
let val = expr;
if Carrier::is_ok(&val) {
    val.unwrap_into_ok()
} else {
    return val.unwrap_into_error();
}
