rust
fn main() {
    type_error(|x| &x);
}

fn type_error<T>(
    _selector:
        for<'a> fn(
            &'a Vec<Box<dyn for<'b> Fn(&'b u8)>>
        ) -> &'a Vec<Box<dyn Fn(T)>>
) {}
