
pub fn foo(params: Option<&[&str]>) {}

fn main() {
    let name = "Foo";
    let msg = foo(Some(&[name.as_slice()]));
}
