plain

error[E0425]: cannot find value `s` in this scope
   --> library/core/src/num/dec2flt/decimal.rs:209:39
    |
209 |     while let Some((&b'0', s_next)) = s.split_first() {
    |                                       ^ help: a local variable with a similar name exists: `d`
error[E0425]: cannot find value `s` in this scope
   --> library/core/src/num/dec2flt/decimal.rs:210:9
    |
210 |         s = s_next;
210 |         s = s_next;
    |         ^ help: a local variable with a similar name exists: `d`

error[E0425]: cannot find value `s` in this scope
   --> library/core/src/num/dec2flt/decimal.rs:213:37
    |
213 |     while let Some((&ch, s_next)) = s.split_first() {
    |                                     ^ help: a local variable with a similar name exists: `d`
error[E0425]: cannot find value `s` in this scope
   --> library/core/src/num/dec2flt/decimal.rs:217:13
    |
217 |             s = s_next;
217 |             s = s_next;
    |             ^ help: a local variable with a similar name exists: `d`

error[E0425]: cannot find value `s` in this scope
   --> library/core/src/num/dec2flt/decimal.rs:223:35
    |
223 |     if let Some((b'.', s_next)) = s.split_first() {
    |                                   ^ help: a local variable with a similar name exists: `d`
error[E0425]: cannot find value `s` in this scope
   --> library/core/src/num/dec2flt/decimal.rs:224:9
    |
224 |         s = s_next;
---

error[E0425]: cannot find value `s` in this scope
   --> library/core/src/num/dec2flt/decimal.rs:228:47
    |
228 |             while let Some((&b'0', s_next)) = s.split_first() {
    |                                               ^ help: a local variable with a similar name exists: `d`
error[E0425]: cannot find value `s` in this scope
   --> library/core/src/num/dec2flt/decimal.rs:229:17
    |
229 |                 s = s_next;
229 |                 s = s_next;
    |                 ^ help: a local variable with a similar name exists: `d`

error[E0425]: cannot find value `s` in this scope
   --> library/core/src/num/dec2flt/decimal.rs:232:15
    |
232 |         while s.len() >= 8 && d.num_digits + 8 < Decimal::MAX_DIGITS {
    |               ^ help: a local variable with a similar name exists: `d`
error[E0425]: cannot find value `s` in this scope
   --> library/core/src/num/dec2flt/decimal.rs:233:21
    |
    |
233 |             let v = s.read_u64();
    |                     ^ help: a local variable with a similar name exists: `d`
error[E0425]: cannot find function `is_8digits` in this scope
   --> library/core/src/num/dec2flt/decimal.rs:234:17
    |
    |
234 |             if !is_8digits(v) {
    |
help: consider importing this function
    |
12  | use crate::num::dec2flt::common::is_8digits;
12  | use crate::num::dec2flt::common::is_8digits;
    |

error[E0425]: cannot find value `s` in this scope
   --> library/core/src/num/dec2flt/decimal.rs:239:13
    |
239 |             s = &s[8..];
    |             ^ help: a local variable with a similar name exists: `d`
error[E0425]: cannot find value `s` in this scope
   --> library/core/src/num/dec2flt/decimal.rs:239:18
    |
    |
239 |             s = &s[8..];
    |                  ^ help: a local variable with a similar name exists: `d`
error[E0425]: cannot find value `s` in this scope
   --> library/core/src/num/dec2flt/decimal.rs:241:41
    |
    |
241 |         while let Some((&ch, s_next)) = s.split_first() {
    |                                         ^ help: a local variable with a similar name exists: `d`
error[E0425]: cannot find value `s` in this scope
   --> library/core/src/num/dec2flt/decimal.rs:245:17
    |
245 |                 s = s_next;
245 |                 s = s_next;
    |                 ^ help: a local variable with a similar name exists: `d`

error[E0425]: cannot find value `s` in this scope
   --> library/core/src/num/dec2flt/decimal.rs:250:27
    |
250 |         d.decimal_point = s.len() as i32 - first.len() as i32;
    |                           ^ help: a local variable with a similar name exists: `d`
error[E0425]: cannot find value `s` in this scope
   --> library/core/src/num/dec2flt/decimal.rs:255:42
    |
    |
255 |         for &c in start[..(start.len() - s.len())].iter().rev() {
    |                                          ^ help: a local variable with a similar name exists: `d`
error[E0425]: cannot find value `s` in this scope
   --> library/core/src/num/dec2flt/decimal.rs:270:34
    |
    |
270 |     if let Some((&ch, s_next)) = s.split_first() {
    |                                  ^ help: a local variable with a similar name exists: `d`
error[E0425]: cannot find value `s` in this scope
   --> library/core/src/num/dec2flt/decimal.rs:272:13
    |
272 |             s = s_next;
272 |             s = s_next;
    |             ^ help: a local variable with a similar name exists: `d`

error[E0425]: cannot find value `s` in this scope
   --> library/core/src/num/dec2flt/decimal.rs:274:42
    |
274 |             if let Some((&ch, s_next)) = s.split_first() {
    |                                          ^ help: a local variable with a similar name exists: `d`
error[E0425]: cannot find value `s` in this scope
   --> library/core/src/num/dec2flt/decimal.rs:277:21
    |
277 |                     s = s_next;
277 |                     s = s_next;
    |                     ^ help: a local variable with a similar name exists: `d`

error[E0425]: cannot find value `s` in this scope
   --> library/core/src/num/dec2flt/decimal.rs:282:45
    |
282 |             while let Some((&ch, s_next)) = s.split_first() {
    |                                             ^ help: a local variable with a similar name exists: `d`
error[E0425]: cannot find value `s` in this scope
   --> library/core/src/num/dec2flt/decimal.rs:288:21
    |
288 |                     s = s_next;
288 |                     s = s_next;
    |                     ^ help: a local variable with a similar name exists: `d`

error[E0425]: cannot find value `integer` in this scope
   --> library/core/src/num/dec2flt/mod.rs:268:39
    |
268 |         fp = parse_long_mantissa::<F>(integer, fractional, exponent);

error[E0425]: cannot find value `fractional` in this scope
   --> library/core/src/num/dec2flt/mod.rs:268:48
    |
    |
268 |         fp = parse_long_mantissa::<F>(integer, fractional, exponent);

error[E0425]: cannot find value `exponent` in this scope
   --> library/core/src/num/dec2flt/mod.rs:268:60
    |
    |
268 |         fp = parse_long_mantissa::<F>(integer, fractional, exponent);

For more information about this error, try `rustc --explain E0425`.
error: could not compile `core` due to 28 previous errors
Build completed unsuccessfully in 0:00:11
