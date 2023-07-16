
#![feature(plugin, custom_derive, decl_macro)]
#![plugin(rocket_codegen)]

#[get"x"]
fn main() {
	print!("hello world")
}
