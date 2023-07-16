
pub fn options() -> OpenOptions {
  OpenOptions::new()
}

let options = File::options();
let file = File::options().read(true).create(true).open("foo.txt");
