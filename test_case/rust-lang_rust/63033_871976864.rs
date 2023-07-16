rust
async fn read_loop(
    &self,
    st: &mut State,
    stream: Box<dyn std::io::Read>
 ) -> Result<(), Error> {
.....
}
