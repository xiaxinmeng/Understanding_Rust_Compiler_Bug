rust

fn get_some_string() -> Option<String> { Some(String::from("foo")) }
fn get_some_int() -> Option<i32> { Some(42) }

fn foo() -> MyResult {
    let some_string = e!(get_some_string());
    let some_int = e!(get_some_int());

    MyResult(Ok(42))
}
