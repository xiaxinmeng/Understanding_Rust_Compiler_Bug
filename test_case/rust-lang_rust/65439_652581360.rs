
pub fn builder() -> OpenOptions {
  OpenOptions::new()
}

let options = File::builder();
let file = File::builder().read(true).create(true).open("foo.txt");
