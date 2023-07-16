
error: multiple patterns overlap on their endpoints
  --> $DIR/overlapping_range_endpoints.rs:16:22
   |
LL |     m!(0u8, 30..=40, 20..=30);
   |             -------  ^^^^^^^ ... with this range
   |             |
   |             this range overlaps on `30_u8`...
   |
   = note: you likely meant to write mutually exclusive ranges
