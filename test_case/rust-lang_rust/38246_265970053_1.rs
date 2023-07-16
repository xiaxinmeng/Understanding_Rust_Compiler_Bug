plain
warning: function is never used: `mk_datetime`, #[warn(dead_code)] on by default
   --> tests/types_roundtrip.rs:127:1
    |
127 |   pub fn mk_datetime(data: (i64, u32)) -> DateTime<UTC> {
    |  _^ starting here...
128 | |     DateTime::from_utc(mk_naive_datetime(data), UTC)
129 | | }
    | |_^ ...ending here
