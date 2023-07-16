
pub fn hello_world<'a>(hello: &'a String, world: &'a String) -> impl Future<Output=()> + 'a {
    async move {
        println!("Hello world: {:?} {:?}", hello, world);
    }
}
