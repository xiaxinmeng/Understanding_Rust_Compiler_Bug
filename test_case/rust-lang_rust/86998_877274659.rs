plain
   Compiling cc v1.0.68
    Checking core v0.0.0 (/checkout/library/core)
   Compiling libc v0.2.93
   Compiling std v0.0.0 (/checkout/library/std)
error: cannot find a built-in macro with name `const_format_args`
    |
839 | /     macro_rules! const_format_args {
839 | /     macro_rules! const_format_args {
840 | |         ($fmt:expr) => {{ /* compiler built-in */ }};
841 | |         ($fmt:expr, $($args:tt)*) => {{ /* compiler built-in */ }};
    | |_____^

   Compiling compiler_builtins v0.1.46
   Compiling unwind v0.0.0 (/checkout/library/unwind)
---
12  |  |     };
13  |  | }
    |  |_- in this expansion of `panic!` (#1)
...
839 | /      macro_rules! const_format_args {
840 | |          ($fmt:expr) => {{ /* compiler built-in */ }};
841 | |          ($fmt:expr, $($args:tt)*) => {{ /* compiler built-in */ }};
    | |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Arguments`, found `()`
    | |______- in this expansion of `$crate::const_format_args!` (#3)
    | 
   ::: library/core/src/panic.rs:13:1
    |
---
849 |        assert!(
    |  ______-
    | |______|
    | |
850 | |          radix >= 2 && radix <= 36,
851 | |          "from_str_radix_int: must lie in the range `[2, 36]` - found {}",
852 | |          radix
    | |       -
    | |_______|
    | |_______in this macro invocation (#1)
    |         in this macro invocation (#2)
---
12  |  |     };
13  |  | }
    |  |_- in this expansion of `panic!` (#1)
...
839 | /      macro_rules! const_format_args {
840 | |          ($fmt:expr) => {{ /* compiler built-in */ }};
841 | |          ($fmt:expr, $($args:tt)*) => {{ /* compiler built-in */ }};
    | |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Arguments`, found `()`
    | |______- in this expansion of `$crate::const_format_args!` (#3)
    | 
    | 
   ::: library/core/src/num/dec2flt/rawfp.rs:253:9
    |
253 |            panic!("fp_to_float: exponent {} too large", e)
    |            |
    |            in this macro invocation (#1)
    |            in this macro invocation (#2)
    | 
---
12  |  |     };
13  |  | }
    |  |_- in this expansion of `panic!` (#1)
...
839 | /      macro_rules! const_format_args {
840 | |          ($fmt:expr) => {{ /* compiler built-in */ }};
841 | |          ($fmt:expr, $($args:tt)*) => {{ /* compiler built-in */ }};
    | |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Arguments`, found `()`
    | |______- in this expansion of `$crate::const_format_args!` (#3)
    | 
    | 
   ::: library/core/src/num/dec2flt/rawfp.rs:257:9
    |
257 |            panic!("fp_to_float: exponent {} too small", e)
    |            |
    |            in this macro invocation (#1)
    |            in this macro invocation (#2)
    | 
---
12   |  |     };
13   |  | }
     |  |_- in this expansion of `panic!` (#1)
...
839  | /      macro_rules! const_format_args {
840  | |          ($fmt:expr) => {{ /* compiler built-in */ }};
841  | |          ($fmt:expr, $($args:tt)*) => {{ /* compiler built-in */ }};
     | |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Arguments`, found `()`
     | |______- in this expansion of `$crate::const_format_args!` (#3)
     | 
    ::: library/core/src/panic.rs:13:1
     |
---
1635 |            _ => panic!(
     |  _______________-
     | |_______________|
     | |
1636 | |              "encode_utf8: need {} bytes to encode U+{:X}, but the buffer has {}",
1637 | |              len,
1638 | |              code,
1639 | |              dst.len(),
     | |          -
     | |__________|
     | |__________in this macro invocation (#1)
     |            in this macro invocation (#2)
---
12   |  |     };
13   |  | }
     |  |_- in this expansion of `panic!` (#1)
...
839  | /      macro_rules! const_format_args {
840  | |          ($fmt:expr) => {{ /* compiler built-in */ }};
841  | |          ($fmt:expr, $($args:tt)*) => {{ /* compiler built-in */ }};
     | |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Arguments`, found `()`
     | |______- in this expansion of `$crate::const_format_args!` (#3)
     | 
    ::: library/core/src/panic.rs:13:1
     |
---
1672 |                panic!(
     |  ______________-
     | |______________|
     | |
1673 | |                  "encode_utf16: need {} units to encode U+{:X}, but the buffer has {}",
1674 | |                  from_u32_unchecked(code).len_utf16(),
1675 | |                  code,
1676 | |                  dst.len(),
     | |              -
     | |______________|
     | |______________in this macro invocation (#1)
     |                in this macro invocation (#2)
---
    |  __-
    | |__|
    | |
584 | |      () => ({
585 | |          $crate::panic!("internal error: entered unreachable code")
586 | |      });
587 | |      ($msg:expr $(,)?) => ({
588 | |          $crate::unreachable!("{}", $msg)
...   |
...   |
591 | |          $crate::panic!($crate::concat!("internal error: entered unreachable code: ", $fmt), $($arg)*)
    | |          |
    | |          in this macro invocation (#3)
    | |          in this macro invocation (#4)
592 | |      });
592 | |      });
593 | |  }
    | |  -
    | |__|
    | |__in this expansion of `unreachable!` (#1)
    |    in this expansion of `$crate::unreachable!` (#2)
839 | /      macro_rules! const_format_args {
839 | /      macro_rules! const_format_args {
840 |            ($fmt:expr) => {{ /* compiler built-in */ }};
841 |            ($fmt:expr, $($args:tt)*) => {{ /* compiler built-in */ }};
    |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Arguments`, found `()`
    | |______- in this expansion of `$crate::const_format_args!` (#5)
    | 
   ::: library/core/src/iter/adapters/zip.rs:210:9
    |
    |
210 |            unreachable!("Always specialized");
    | 
   ::: library/core/src/panic.rs:13:1
    |
13  | /  pub macro panic_2015 {
---
     |  __-
     | |__|
     | |
584  | |      () => ({
585  | |          $crate::panic!("internal error: entered unreachable code")
586  | |      });
587  | |      ($msg:expr $(,)?) => ({
588  | |          $crate::unreachable!("{}", $msg)
...    |
...    |
591  | |          $crate::panic!($crate::concat!("internal error: entered unreachable code: ", $fmt), $($arg)*)
     | |          |
     | |          in this macro invocation (#3)
     | |          in this macro invocation (#4)
592  | |      });
592  | |      });
593  | |  }
     | |  -
     | |__|
     | |__in this expansion of `unreachable!` (#1)
     |    in this expansion of `$crate::unreachable!` (#2)
839  | /      macro_rules! const_format_args {
839  | /      macro_rules! const_format_args {
840  |            ($fmt:expr) => {{ /* compiler built-in */ }};
841  |            ($fmt:expr, $($args:tt)*) => {{ /* compiler built-in */ }};
     |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Arguments`, found `()`
     | |______- in this expansion of `$crate::const_format_args!` (#5)
     | 
    ::: library/core/src/iter/traits/iterator.rs:3468:9
     |
     |
3468 |            unreachable!("Always specialized");
     | 
    ::: library/core/src/panic.rs:13:1
     |
13   | /  pub macro panic_2015 {
---
12   |  |     };
13   |  | }
     |  |_- in this expansion of `panic!` (#1)
...
839  | /      macro_rules! const_format_args {
840  | |          ($fmt:expr) => {{ /* compiler built-in */ }};
841  | |          ($fmt:expr, $($args:tt)*) => {{ /* compiler built-in */ }};
     | |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Arguments`, found `()`
     | |______- in this expansion of `$crate::const_format_args!` (#3)
     | 
    ::: library/core/src/option.rs:1243:5
     |
---
12  |  |     };
13  |  | }
    |  |_- in this expansion of `panic!` (#1)
...
839 | /      macro_rules! const_format_args {
840 | |          ($fmt:expr) => {{ /* compiler built-in */ }};
841 | |          ($fmt:expr, $($args:tt)*) => {{ /* compiler built-in */ }};
    | |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Arguments`, found `()`
    | |______- in this expansion of `$crate::const_format_args!` (#3)
    | 
   ::: library/core/src/panicking.rs:69:5
    |
    |
69  |        panic!("index out of bounds: the len is {} but the index is {}", len, index)
    |        |
    |        in this macro invocation (#1)
    |        in this macro invocation (#2)
    | 
---
12  |  |     };
13  |  | }
    |  |_- in this expansion of `panic!` (#1)
...
839 | /      macro_rules! const_format_args {
840 | |          ($fmt:expr) => {{ /* compiler built-in */ }};
841 | |          ($fmt:expr, $($args:tt)*) => {{ /* compiler built-in */ }};
    | |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Arguments`, found `()`
    | |______- in this expansion of `$crate::const_format_args!` (#3)
    | 
   ::: library/core/src/panic.rs:13:1
    |
---
    | |__- in this expansion of `$crate::panic::panic_2015!` (#2)
    | 
   ::: library/core/src/panicking.rs:166:23
    |
166 |            Some(args) => panic!(
    | |________________________|
    | |
    | |
167 | |              r#"assertion failed: `(left {} right)`
168 | |    left: `{:?}`,
169 | |   right: `{:?}`: {}"#,
170 | |              op, left, right, args
    | |          -
    | |__________|
    | |__________in this macro invocation (#1)
    |            in this macro invocation (#2)
---
12  |  |     };
13  |  | }
    |  |_- in this expansion of `panic!` (#1)
...
839 | /      macro_rules! const_format_args {
840 | |          ($fmt:expr) => {{ /* compiler built-in */ }};
841 | |          ($fmt:expr, $($args:tt)*) => {{ /* compiler built-in */ }};
    | |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Arguments`, found `()`
    | |______- in this expansion of `$crate::const_format_args!` (#3)
    | 
   ::: library/core/src/panic.rs:13:1
    |
---
172 |            None => panic!(
    |  __________________-
    | |__________________|
    | |
173 | |              r#"assertion failed: `(left {} right)`
174 | |    left: `{:?}`,
175 | |   right: `{:?}`"#,
177 | |          ),
    | |          -
    | |__________|
    | |__________in this macro invocation (#1)
---
12   |  |     };
13   |  | }
     |  |_- in this expansion of `panic!` (#1)
...
839  | /      macro_rules! const_format_args {
840  | |          ($fmt:expr) => {{ /* compiler built-in */ }};
841  | |          ($fmt:expr, $($args:tt)*) => {{ /* compiler built-in */ }};
     | |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Arguments`, found `()`
     | |______- in this expansion of `$crate::const_format_args!` (#3)
     | 
    ::: library/core/src/result.rs:1355:5
     |
     |
1355 |        panic!("{}: {:?}", msg, error)
     |        |
     |        in this macro invocation (#1)
     |        in this macro invocation (#2)
     | 
---
12  |  |     };
13  |  | }
    |  |_- in this expansion of `panic!` (#2)
...
839 | /      macro_rules! const_format_args {
840 | |          ($fmt:expr) => {{ /* compiler built-in */ }};
841 | |          ($fmt:expr, $($args:tt)*) => {{ /* compiler built-in */ }};
    | |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Arguments`, found `()`
    | |______- in this expansion of `$crate::const_format_args!` (#4)
    | 
   ::: library/core/src/fmt/num.rs:130:1
    |
    |
130 | /  macro_rules! radix {
131 | |      ($T:ident, $base:expr, $prefix:expr, $($x:pat => $conv:expr),+) => {
132 | |          impl GenericRadix for $T {
133 | |              const BASE: u8 = $base;
...   |
138 | |                      x => panic!("number not in the range 0..={}: {}", Self::BASE - 1, x),
    | |                           |
    | |                           in this macro invocation (#2)
    | |                           in this macro invocation (#3)
...   |
...   |
142 | |      }
143 | |  }
    | |__- in this expansion of `radix!` (#1)
144 | 
145 |    radix! { Binary,    2, "0b", x @  0 ..=  1 => b'0' + x }
    | 
   ::: library/core/src/panic.rs:13:1
    |
13  | /  pub macro panic_2015 {
---
12  |  |     };
13  |  | }
    |  |_- in this expansion of `panic!` (#2)
...
839 | /      macro_rules! const_format_args {
840 | |          ($fmt:expr) => {{ /* compiler built-in */ }};
841 | |          ($fmt:expr, $($args:tt)*) => {{ /* compiler built-in */ }};
    | |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Arguments`, found `()`
    | |______- in this expansion of `$crate::const_format_args!` (#4)
    | 
   ::: library/core/src/fmt/num.rs:130:1
    |
    |
130 | /  macro_rules! radix {
131 | |      ($T:ident, $base:expr, $prefix:expr, $($x:pat => $conv:expr),+) => {
132 | |          impl GenericRadix for $T {
133 | |              const BASE: u8 = $base;
...   |
138 | |                      x => panic!("number not in the range 0..={}: {}", Self::BASE - 1, x),
    | |                           |
    | |                           in this macro invocation (#2)
    | |                           in this macro invocation (#3)
...   |
...   |
142 | |      }
143 | |  }
    | |__- in this expansion of `radix!` (#1)
...
146 |    radix! { Octal,     8, "0o", x @  0 ..=  7 => b'0' + x }
    | 
   ::: library/core/src/panic.rs:13:1
    |
13  | /  pub macro panic_2015 {
---
12  |  |     };
13  |  | }
    |  |_- in this expansion of `panic!` (#2)
...
839 | /      macro_rules! const_format_args {
840 | |          ($fmt:expr) => {{ /* compiler built-in */ }};
841 | |          ($fmt:expr, $($args:tt)*) => {{ /* compiler built-in */ }};
    | |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Arguments`, found `()`
    | |______- in this expansion of `$crate::const_format_args!` (#4)
    | 
   ::: library/core/src/fmt/num.rs:130:1
    |
    |
130 | /  macro_rules! radix {
131 | |      ($T:ident, $base:expr, $prefix:expr, $($x:pat => $conv:expr),+) => {
132 | |          impl GenericRadix for $T {
133 | |              const BASE: u8 = $base;
...   |
138 | |                      x => panic!("number not in the range 0..={}: {}", Self::BASE - 1, x),
    | |                           |
    | |                           in this macro invocation (#2)
    | |                           in this macro invocation (#3)
...   |
...   |
142 | |      }
143 | |  }
    | |__- in this expansion of `radix!` (#1)
...
147 |    radix! { LowerHex, 16, "0x", x @  0 ..=  9 => b'0' + x, x @ 10 ..= 15 => b'a' + (x - 10) }
    | 
   ::: library/core/src/panic.rs:13:1
    |
13  | /  pub macro panic_2015 {
---
12  |  |     };
13  |  | }
    |  |_- in this expansion of `panic!` (#2)
...
839 | /      macro_rules! const_format_args {
840 | |          ($fmt:expr) => {{ /* compiler built-in */ }};
841 | |          ($fmt:expr, $($args:tt)*) => {{ /* compiler built-in */ }};
    | |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Arguments`, found `()`
    | |______- in this expansion of `$crate::const_format_args!` (#4)
    | 
   ::: library/core/src/fmt/num.rs:130:1
    |
    |
130 | /  macro_rules! radix {
131 | |      ($T:ident, $base:expr, $prefix:expr, $($x:pat => $conv:expr),+) => {
132 | |          impl GenericRadix for $T {
133 | |              const BASE: u8 = $base;
...   |
138 | |                      x => panic!("number not in the range 0..={}: {}", Self::BASE - 1, x),
    | |                           |
    | |                           in this macro invocation (#2)
    | |                           in this macro invocation (#3)
...   |
...   |
142 | |      }
143 | |  }
    | |__- in this expansion of `radix!` (#1)
...
148 |    radix! { UpperHex, 16, "0x", x @  0 ..=  9 => b'0' + x, x @ 10 ..= 15 => b'A' + (x - 10) }
    | 
   ::: library/core/src/panic.rs:13:1
    |
13  | /  pub macro panic_2015 {
---
12   |  |     };
13   |  | }
     |  |_- in this expansion of `panic!` (#1)
...
839  | /      macro_rules! const_format_args {
840  | |          ($fmt:expr) => {{ /* compiler built-in */ }};
841  | |          ($fmt:expr, $($args:tt)*) => {{ /* compiler built-in */ }};
     | |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Arguments`, found `()`
     | |______- in this expansion of `$crate::const_format_args!` (#3)
     | 
    ::: library/core/src/panic.rs:13:1
     |
---
3051 |                panic!(
     |  ______________-
     | |______________|
     | |
3052 | |                  "source slice length ({}) does not match destination slice length ({})",
3053 | |                  src_len, dst_len,
     | |               -
     | |_______________|
     | |_______________in this macro invocation (#1)
     |                 in this macro invocation (#2)
---
12  |  |     };
13  |  | }
    |  |_- in this expansion of `panic!` (#1)
...
839 | /      macro_rules! const_format_args {
840 | |          ($fmt:expr) => {{ /* compiler built-in */ }};
841 | |          ($fmt:expr, $($args:tt)*) => {{ /* compiler built-in */ }};
    | |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Arguments`, found `()`
    | |______- in this expansion of `$crate::const_format_args!` (#3)
    | 
   ::: library/core/src/slice/index.rs:34:5
    |
    |
34  |        panic!("range start index {} out of range for slice of length {}", index, len);
    |        |
    |        in this macro invocation (#1)
    |        in this macro invocation (#2)
    | 
---
12  |  |     };
13  |  | }
    |  |_- in this expansion of `panic!` (#1)
...
839 | /      macro_rules! const_format_args {
840 | |          ($fmt:expr) => {{ /* compiler built-in */ }};
841 | |          ($fmt:expr, $($args:tt)*) => {{ /* compiler built-in */ }};
    | |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Arguments`, found `()`
    | |______- in this expansion of `$crate::const_format_args!` (#3)
    | 
   ::: library/core/src/slice/index.rs:41:5
    |
    |
41  |        panic!("range end index {} out of range for slice of length {}", index, len);
    |        |
    |        in this macro invocation (#1)
    |        in this macro invocation (#2)
    | 
---
12  |  |     };
13  |  | }
    |  |_- in this expansion of `panic!` (#1)
...
839 | /      macro_rules! const_format_args {
840 | |          ($fmt:expr) => {{ /* compiler built-in */ }};
841 | |          ($fmt:expr, $($args:tt)*) => {{ /* compiler built-in */ }};
    | |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Arguments`, found `()`
    | |______- in this expansion of `$crate::const_format_args!` (#3)
    | 
   ::: library/core/src/slice/index.rs:48:5
    |
    |
48  |        panic!("slice index starts at {} but ends at {}", index, end);
    |        |
    |        in this macro invocation (#1)
    |        in this macro invocation (#2)
    | 
---
12  |  |     };
13  |  | }
    |  |_- in this expansion of `panic!` (#1)
...
839 | /      macro_rules! const_format_args {
840 | |          ($fmt:expr) => {{ /* compiler built-in */ }};
841 | |          ($fmt:expr, $($args:tt)*) => {{ /* compiler built-in */ }};
    | |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Arguments`, found `()`
    | |______- in this expansion of `$crate::const_format_args!` (#3)
    | 
   ::: library/core/src/slice/sort.rs:843:9
    |
    |
843 |            panic!("partition_at_index index {} greater than length of slice {}", index, v.len());
    |            |
    |            in this macro invocation (#1)
    |            in this macro invocation (#2)
    | 
---
12  |  |     };
13  |  | }
    |  |_- in this expansion of `panic!` (#1)
...
839 | /      macro_rules! const_format_args {
840 | |          ($fmt:expr) => {{ /* compiler built-in */ }};
841 | |          ($fmt:expr, $($args:tt)*) => {{ /* compiler built-in */ }};
    | |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Arguments`, found `()`
    | |______- in this expansion of `$crate::const_format_args!` (#3)
    | 
   ::: library/core/src/str/mod.rs:91:9
    |
    |
91  |            panic!("byte index {} is out of bounds of `{}`{}", oob_index, s_trunc, ellipsis);
    |            |
    |            in this macro invocation (#1)
    |            in this macro invocation (#2)
    | 
---
12  |  |     };
13  |  | }
    |  |_- in this expansion of `panic!` (#1)
...
839 | /      macro_rules! const_format_args {
840 | |          ($fmt:expr) => {{ /* compiler built-in */ }};
841 | |          ($fmt:expr, $($args:tt)*) => {{ /* compiler built-in */ }};
    | |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Arguments`, found `()`
    | |______- in this expansion of `$crate::const_format_args!` (#3)
    | 
   ::: library/core/src/panic.rs:13:1
    |
---
95  |        assert!(
    |  ______-
    | |______|
    | |
96  | |          begin <= end,
97  | |          "begin <= end ({} <= {}) when slicing `{}`{}",
98  | |          begin,
101 | |          ellipsis
102 | |      );
    | |       -
    | |_______|
---
12  |  |     };
13  |  | }
    |  |_- in this expansion of `panic!` (#1)
...
839 | /      macro_rules! const_format_args {
840 | |          ($fmt:expr) => {{ /* compiler built-in */ }};
841 | |          ($fmt:expr, $($args:tt)*) => {{ /* compiler built-in */ }};
    | |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Arguments`, found `()`
    | |______- in this expansion of `$crate::const_format_args!` (#3)
    | 
   ::: library/core/src/panic.rs:13:1
    |
---
114 |        panic!(
    |  ______-
    | |______|
    | |
115 | |          "byte index {} is not a char boundary; it is inside {:?} (bytes {:?}) of `{}`{}",
116 | |          index, ch, char_range, s_trunc, ellipsis
    | |       -
    | |_______|
    | |_______in this macro invocation (#1)
    |         in this macro invocation (#2)
