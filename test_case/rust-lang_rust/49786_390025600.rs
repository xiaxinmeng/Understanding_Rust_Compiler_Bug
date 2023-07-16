rust
impl ops::Index<MyIndex> for [String; 2] {
    type Output = String;

    fn index(&self, index: MyIndex) -> &String {
        let this: &[String] = self;
        match index.0 {
            false => &this[0],
            true => &this[1],
        }
    }
}
