rust
use std::io::BufRead;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let gpl3 = std::path::PathBuf::from("/usr/share/common-licenses/GPL-3");
  let file = std::fs::File::open(gpl3)?;
  let mut binding = std::io::BufReader::with_capacity(16384, file); // increase buffer size to 16 KB
  let bytes = binding.fill_buf()?;
  let text = String::from_utf8(bytes.to_vec())?;

  println!("{}", bytes.len());
  println!("{text}");

  Ok(())
}
