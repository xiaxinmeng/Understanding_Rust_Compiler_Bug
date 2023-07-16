
let options = File::options(); // Ok
let options = File::with_options();  // Weird

let file = File::with_options().read(true).create(true).open("foo.txt"); // Ok
let file = File::options().read(true).create(true).open("foo.txt"); // Bit weirder. Maybe it is better actually?
