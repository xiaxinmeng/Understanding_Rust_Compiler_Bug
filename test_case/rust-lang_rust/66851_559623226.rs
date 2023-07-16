
#[derive(Debug, PartialEq)]
enum MyOption {
    Some(u64),
    None,
}

fn main() {
    let res = if let Ok(r) = "1".parse() {
        MyOption::Some(r)
    } else {
        MyOption::None
    };

    assert_eq!(MyOption::Some(1), res);
}
