 rust
use std::io::BufferedReader;
use std::io::File;

let path = Path::new("message.txt");
let mut file = BufferedReader::new(File::open(&path));
for line in file.lines() {
    print!("{}", line);
}
