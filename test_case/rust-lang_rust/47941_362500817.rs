rust
#![feature(proc_macro)]

extern crate repro;
use repro::AsChangeset;

#[derive(AsChangeset)]
#[table_name = "users"]
struct UserForm {
    id: i32,
    name: String,
}

fn main() {}
