 rs
// uses fictitious IO APIs as an example
fn readlines(filename: &str) -> Result<~[~str], IoError> {
  let file = try!(files::open(filename));
  let body = try!(file.read());
  body.split('\n').collect()
}
