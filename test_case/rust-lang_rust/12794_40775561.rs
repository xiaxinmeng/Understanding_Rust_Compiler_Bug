 rs
// ...
// tries to read field before read_option
self.command = try!(d.read_struct_field("command", 3u, |d| Decodable::decode(d)));
// ..
