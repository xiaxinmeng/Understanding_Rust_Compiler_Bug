rust
async fn foo() {}

fn main() {
    std::mem::size_of_val(foo());
    //                    ^ if we insert the needed `&` here, everything compiles
}
