
         .           macro_rules! read_uleb128 {
          .               ($dec:expr, $fun:ident) => {{
100,680,777 ( 1.13%)          let (value, bytes_read) = leb128::$fun(&$dec.data[$dec.position..]);
 67,858,196 ( 0.76%)          $dec.position += bytes_read;
 43,378,625 ( 0.49%)          Ok(value)
          .               }};
          .           }
