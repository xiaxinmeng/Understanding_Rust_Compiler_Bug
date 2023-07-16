rust
if let pat = expr1 {...} else expr2
// can be lowered to
match expr1 {
    pat => {...}
    _ => expr2
}
