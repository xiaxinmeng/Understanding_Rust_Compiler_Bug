 rust
let mut r = some_reader;
let len = r.read_be_i32() as uint;
let some_struct = read_some_struct(LimitedReader::new(len, &mut r));
...
