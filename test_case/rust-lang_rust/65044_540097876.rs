
trait UseString: GetString {
    fn use_string(&self) {
        println!("{:?}", self.get_a());
    }
}
