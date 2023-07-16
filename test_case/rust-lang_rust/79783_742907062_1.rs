rust
impl ServiceModule for Http {
    async fn async_entry (&self) {
        // if that code in there, I can't compile
    }
}

#[async_std::main]
async fn main () {
    // if that code in there, everything is okay
}
