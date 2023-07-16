rust
let f = File::with_options("file.txt", |o| o.read(true).create(true));
