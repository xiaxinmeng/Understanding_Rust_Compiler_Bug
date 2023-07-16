rust
extern crate serde_json;

fn innocent_bystander() {
    // Approximately what assert_eq! does, inlined for the sake of simplicity
    if "x".as_bytes() != &[] {
        panic!();
    }
}
