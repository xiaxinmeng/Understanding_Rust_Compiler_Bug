 rust
fn f(s: &str, fc: char, lc: char) {
    assert!(fc == s.trim_left().chars().next().unwrap());
    assert!(lc == s.trim_right().chars().last().unwrap());
}

fn main() {
    f("  English  ", 'E', 'h');
    f("  עברית  ",
'ע',
'ת');
}
