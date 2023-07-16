rust
let mut header = MyHeaderData::default();
let mut pixel_data: &[u8] = &[];
let mut line_it = file_data.split(|&u| u == b'\n');
while let Some(line) = line_it.next() {
  // handle the header data by filling in the struct
  if all_header_data_has_been_parsed {
    pixel_data = line_it.remainder();
    break;
  }
}
// from here we know if we'll need to parse `pixel_data` as ascii or as binary
