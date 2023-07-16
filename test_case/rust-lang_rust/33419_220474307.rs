 rust
/** Output from `top -b | grep sort_by_key_mem`
  PID   Memory  in GB  CPU% Mem%   SysTime State              Name
 6515  1557.5m 1.006g   8.6  5.2   0:00.13 S                  `- sort_by_key_mem
 6515  1557.5m 1.006g   0.0  5.2   0:00.13 S                  `- sort_by_key_mem
 6515  3605.5m 3.006g  63.6 15.4   0:01.11 R                  `- sort_by_key_mem
 6515  3605.5m 3.006g 100.0 15.4   0:02.62 R                  `- sort_by_key_mem
 6515  3605.5m 1.006g  49.0  5.2   0:03.36 S                  `- sort_by_key_mem
 6515  3605.5m 1.006g   0.0  5.2   0:03.36 S                  `- sort_by_key_memb
*/

fn main() {
  let size : i64 = 6* 1024 * 1024;

  let mut v = Vec::with_capacity(size as usize);

  let mut i = -size;
  while i < size {
    v.push( BigStruct::new(i as u64) );
    i += 3;
  }

  println!("{:?}", v[0]);
  std::thread::sleep( std::time::Duration::from_secs(6) );

  v.sort_by_key(|k| k.a00);

  println!("{:?}", v[0]);
  std::thread::sleep( std::time::Duration::from_secs(6) );
}

#[derive(Ord, Eq, PartialEq, PartialOrd, Debug)]
struct BigStruct{
  a00: u64,
  a01: u64,
  a02: u64,
  a03: u64,
  a04: u64,
  a05: u64,
  a06: u64,
  a07: u64,
  a08: u64,
  a09: u64,
  a0A: u64,
  a0B: u64,
  a0C: u64,
  a0D: u64,
  a0E: u64,
  a0F: u64,

  a10: u64,
  a11: u64,
  a12: u64,
  a13: u64,
  a14: u64,
  a15: u64,
  a16: u64,
  a17: u64,
  a18: u64,
  a19: u64,
  a1A: u64,
  a1B: u64,
  a1C: u64,
  a1D: u64,
  a1E: u64,
  a1F: u64,
}

impl BigStruct {
  pub fn new(i : u64) -> BigStruct {
    BigStruct{
      a00: i,
      a01: i,
      a02: i,
      a03: i,
      a04: i,
      a05: i,
      a06: i,
      a07: i,
      a08: i,
      a09: i,
      a0A: i,
      a0B: i,
      a0C: i,
      a0D: i,
      a0E: i,
      a0F: i,

      a10: i,
      a11: i,
      a12: i,
      a13: i,
      a14: i,
      a15: i,
      a16: i,
      a17: i,
      a18: i,
      a19: i,
      a1A: i,
      a1B: i,
      a1C: i,
      a1D: i,
      a1E: i,
      a1F: i,
    }
  }
}
