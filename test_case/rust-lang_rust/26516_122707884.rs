 rust
struct Foo;

fn get_res() -> Result<&'static str, Foo> { Ok("ok") }

fn main() {
    let res1 = get_res();
    assert!(res1.is_ok());
    assert_eq!("just for test", res1.unwrap());
}
