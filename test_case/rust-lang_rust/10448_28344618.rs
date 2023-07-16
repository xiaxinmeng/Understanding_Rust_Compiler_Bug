 rust
ret_type_infer!(fn foo(x: int, y: int) -> _ { <body> })

// expands to

fn foo(x: int, y: int) -> typeof({ <body> }) { <body> }
