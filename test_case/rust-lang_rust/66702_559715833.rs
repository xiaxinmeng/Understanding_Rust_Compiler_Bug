rust
fn main() {
    'some_label: loop {
        || break 'some_label (); 
    }
}
