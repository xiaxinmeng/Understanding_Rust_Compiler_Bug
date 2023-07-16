rust
impl Clone for Box<dyn Value> {
    fn clone(&self) -> Box<dyn Value> {
        self.box_clone() // THIS LINE is main.rs:37
    }
}
