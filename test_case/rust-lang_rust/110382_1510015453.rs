plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0425]: cannot find value `x` in this scope
  --> library/core/src/num/dec2flt/parse.rs:87:40
   |
87 |     if matches!(s.first(), Some(&x) if x.is_ascii_digit()) {
   |                                        ^ help: a local variable with a similar name exists: `s`
error[E0425]: cannot find value `b` in this scope
    --> library/core/src/net/ip_addr.rs:1366:76
     |
     |
1366 |             || (matches!(self.segments(), [0x2001, b, _, _, _, _, _, _] if b < 0x200)

error[E0425]: cannot find value `b` in this scope
    --> library/core/src/net/ip_addr.rs:1377:83
     |
     |
1377 |                     || matches!(self.segments(), [0x2001, b, _, _, _, _, _, _] if b >= 0x20 && b <= 0x2F)

error[E0425]: cannot find value `b` in this scope
    --> library/core/src/net/ip_addr.rs:1377:96
     |
     |
1377 |                     || matches!(self.segments(), [0x2001, b, _, _, _, _, _, _] if b >= 0x20 && b <= 0x2F)


error[E0425]: cannot find value `j` in this scope
    |
    |
125 |         matches!(self.into_searcher(haystack).next_back(), SearchStep::Match(_, j) if haystack.len() == j)

For more information about this error, try `rustc --explain E0425`.
error: could not compile `core` due to 5 previous errors
Build completed unsuccessfully in 0:03:14
