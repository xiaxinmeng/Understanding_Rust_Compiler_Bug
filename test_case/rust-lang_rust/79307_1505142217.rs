
warning: multiple patterns overlap on their endpoints
 --> src/main.rs:4:5
  |
3 |     '\u{0}'..='\u{D7FF}' | '\u{E000}'..='\u{10_FFFF}' => {},
  |     --------------------   -------------------------- this range overlaps on `'\u{e000}'`...
  |     |
  |     this range overlaps on `'\u{d7ff}'`...
4 |     '\u{D7FF}'..='\u{E000}' => {}, // should be unreachable
  |     ^^^^^^^^^^^^^^^^^^^^^^^ ... with this range
  |
  = note: you likely meant to write mutually exclusive ranges
  = note: `#[warn(overlapping_range_endpoints)]` on by default
  