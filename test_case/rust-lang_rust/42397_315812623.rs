rust
impl<T: Send> Sender<T> {
    pub fn shared(self) -> SharedSender<T> {
        // upgrade to Flavor::Shared, take shared::Packet, create SharedSender
    }
}
