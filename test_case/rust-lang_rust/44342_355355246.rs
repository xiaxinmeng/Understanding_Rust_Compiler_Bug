rust
// Produces warning: unused macro definition
macro_rules! assert_ok {
    ($s:expr, $res:expr, $rest:expr) => {
        assert_eq!($s, Ok(($res, $rest)))
    };
}

#[test]
fn test() {
    assert_ok!(Ok(1, ""), 1, "");
}

fn main() {}
