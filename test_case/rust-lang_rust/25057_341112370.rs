
cat > foo.rs <<EOF
#[no_mangle]
pub extern fn foo() {}
fn main() {foo();}
EOF
rustc foo.rs
nm -g foo | grep foo
