rust
#[wasm_import_section = "somewhere else"]
extern {
    #[link_name = "some_other_name"]
    fn foo();
    fn bar();
}
