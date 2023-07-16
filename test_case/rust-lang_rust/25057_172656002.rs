
cat > foo.rs <<EOF
#[no_mangle]
pub extern fn foo() {}
fn main() {}
EOF
rustc foo.rs
nm -g foo | grep foo
# should see the symbol `foo` defined here
