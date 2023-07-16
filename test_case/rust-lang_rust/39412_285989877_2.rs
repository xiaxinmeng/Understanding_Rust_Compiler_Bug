rust
mod foo {
    pub macro ignorant_macro() {
        let i = 0;
        println!("{}", i); // Without `let i = 0;`, there would be no `i` in scope here.
    }
}

// Unsuspecting user's code
#[allow(non_camel_case_types)]
struct i(i64);

fn main() {
    foo::ignorant_macro!();
}
