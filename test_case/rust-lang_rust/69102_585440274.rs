rust
pub struct SimpleObject {
    name: RefCell<Option<String>>,
    constructed: RefCell<bool>,
}
