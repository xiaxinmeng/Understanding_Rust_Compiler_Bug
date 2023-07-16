rust
fn parse_the_command(inputs: Inputs) -> Command { /* ... */ }

fn main() {
    let child = parse_the_command(/* ... */).spawn()?;
}

#[test]
fn test_parse_the_command() {
    let expected = /* ... */;
    let actual = parse_the_command(/* ... */);
    assert_eq!(expected, actual);
}
