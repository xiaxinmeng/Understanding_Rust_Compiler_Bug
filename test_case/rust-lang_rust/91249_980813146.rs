rust
#![feature(decl_macro)]

pub macro generate_struct_new($name: ident) {
    pub struct $name {
        x: i32
    }

    pub fn make_struct(x: i32) -> $name {
        $name { x }
    }
}

generate_struct_new!(MyTestStruct);

fn main() {
    let instance = make_struct(3);
}
