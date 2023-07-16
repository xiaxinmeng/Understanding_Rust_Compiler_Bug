
error: lifetime may not live long enough
  --> src/main.rs:40:9
   |
36 |       fn from(bounds: (&str, &str)) -> Self {
   |                        -               ---- return type is BytesRange<'2>
   |                        |
   |                        let's call the lifetime of this reference `'1`
...
40 | /         BytesRange {
41 | |             start: OrdBytes(lower.as_bytes()),
42 | |             end: OrdBytes(upper.as_bytes()),
43 | |         }
   | |_________^ associated function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`

error: lifetime may not live long enough
  --> src/main.rs:40:9
   |
36 |       fn from(bounds: (&str, &str)) -> Self {
   |                              -         ---- return type is BytesRange<'2>
   |                              |
   |                              let's call the lifetime of this reference `'3`
...
40 | /         BytesRange {
41 | |             start: OrdBytes(lower.as_bytes()),
42 | |             end: OrdBytes(upper.as_bytes()),
43 | |         }
   | |_________^ associated function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'3`

error: could not compile `playground` due to 2 previous errors
