rust
#[derive(PartialEq)]
struct Parse {
    cmd: OsString,
    args: Vec<OsString>,
}

fn parse_the_command(inputs: Inputs) -> Parse { /* ... */ }

fn main() {
    let parse = parse_the_command(/* ... */);
    let child = Command::new(parse.cmd).args(parse.args).spawn()?;
}

#[test]
fn test_parse_the_command() {
    let expected = /* ... */;
    let actual = parse_the_command(/* ... */);
    assert_eq!(expected, actual);
}
