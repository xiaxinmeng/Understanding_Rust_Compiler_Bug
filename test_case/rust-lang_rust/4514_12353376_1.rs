
impl Actor: Drop {
    fn finalize(&mut self) {
        if failing() {
               let mut saved_port = ... some stand in value ...
               self.port <-> saved_port;
               spawn {
                   Actor::resume(port);
               }
        }
    }
}
