
pub fn with_options() -> OpenOptions {
  OpenOptions::new()
}

let options = File::with_options();
let file = File::with_options().read(true).create(true).open("foo.txt");
