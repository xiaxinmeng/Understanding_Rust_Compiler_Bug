
fn main() {
    let mut chars = "HHeelllloo,,  wwoorrlldd!!".chars();
    for c in chars.by_ref() {
        print!("{}", c);
        chars.next(); // Skip next
    }
}
