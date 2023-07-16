
pub mod foo {
    pub fn bar() {}
    pub fn qux() {}
}

use foo::bar;
fn main() {
    bar::qux();
}
