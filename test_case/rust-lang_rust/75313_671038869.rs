rust
struct Regex;

fn nested(x: (for<'r, 's> fn(&'r Regex, &'s str), String)) -> (fn(&Regex, &str), String) {
    x
}

fn main() {
    nested((|_x, _y| (), String::new()));
}
