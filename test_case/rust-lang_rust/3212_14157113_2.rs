 rust
enum what { }
fn match_with_empty(x: what) -> ~str {
    match (x) { //use parentheses to remove the ambiguity
    }
}
