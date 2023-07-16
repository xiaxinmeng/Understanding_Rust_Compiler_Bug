rust
#![cfg(test)]
#![feature(yeet_expr)]

fn x() -> Result<(), i32> {
    do yeet 0;
}
