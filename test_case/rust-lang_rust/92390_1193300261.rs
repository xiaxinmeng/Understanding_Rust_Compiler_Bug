plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0277]: the trait bound `usize: Drop` is not satisfied
    --> library/core/src/ptr/mod.rs:1671:48
     |
1671 |     let gcdpow = unsafe { cttz_nonzero(stride).min(cttz_nonzero(a)) };
     |                                                ^^^ the trait `Drop` is not implemented for `usize`
note: required by a bound in `cmp::Ord::min`
    --> library/core/src/cmp.rs:830:15
     |
     |
827  |     fn min(self, other: Self) -> Self
     |        --- required by a bound in this
...
830  |         Self: ~const Drop,
     |               ^^^^^^^^^^^ required by this bound in `cmp::Ord::min`

error[E0277]: can't drop `T` in const contexts
     |
     |
1279 |     v1.max(v2)
     |        --- ^^ expected an implementor of trait `~const Destruct`
     |        required by a bound introduced by this call
     |
     |
     = note: the trait bound `T: ~const Destruct` is not satisfied
note: required by a bound in `cmp::Ord::max`
     |
     |
800  |     fn max(self, other: Self) -> Self
     |        --- required by a bound in this
...
803  |         Self: ~const Destruct,
     |               ^^^^^^^^^^^^^^^ required by this bound in `cmp::Ord::max`
     |
     |
1279 |     v1.max(&v2)
     |            +
1279 |     v1.max(&mut v2)

error[E0277]: the trait bound `usize: Drop` is not satisfied
    --> library/core/src/array/iter.rs:289:21
     |
     |
289  |         let delta = cmp::min(n, len);
     |                     ^^^^^^^^ the trait `Drop` is not implemented for `usize`
note: required by a bound in `min`
    --> library/core/src/cmp.rs:1214:34
     |
     |
1214 | pub const fn min<T: ~const Ord + ~const Drop>(v1: T, v2: T) -> T {
     |                                  ^^^^^^^^^^^ required by this bound in `min`
error[E0277]: the trait bound `usize: Drop` is not satisfied
    --> library/core/src/array/iter.rs:344:21
     |
     |
344  |         let delta = cmp::min(n, len);
     |                     ^^^^^^^^ the trait `Drop` is not implemented for `usize`
note: required by a bound in `min`
    --> library/core/src/cmp.rs:1214:34
     |
     |
1214 | pub const fn min<T: ~const Ord + ~const Drop>(v1: T, v2: T) -> T {
     |                                  ^^^^^^^^^^^ required by this bound in `min`
error[E0277]: the trait bound `usize: Drop` is not satisfied
    --> library/core/src/iter/adapters/skip.rs:212:19
     |
     |
212  |         let min = crate::cmp::min(self.len(), n);
     |                   ^^^^^^^^^^^^^^^ the trait `Drop` is not implemented for `usize`
note: required by a bound in `min`
    --> library/core/src/cmp.rs:1214:34
     |
     |
1214 | pub const fn min<T: ~const Ord + ~const Drop>(v1: T, v2: T) -> T {
     |                                  ^^^^^^^^^^^ required by this bound in `min`
error[E0277]: the trait bound `usize: Drop` is not satisfied
    --> library/core/src/iter/adapters/take.rs:65:21
     |
     |
65   |         let lower = cmp::min(lower, self.n);
     |                     ^^^^^^^^ the trait `Drop` is not implemented for `usize`
note: required by a bound in `min`
    --> library/core/src/cmp.rs:1214:34
     |
     |
1214 | pub const fn min<T: ~const Ord + ~const Drop>(v1: T, v2: T) -> T {
     |                                  ^^^^^^^^^^^ required by this bound in `min`
error[E0277]: the trait bound `usize: Drop` is not satisfied
   --> library/core/src/iter/adapters/take.rs:118:30
    |
    |
118 |         let min = self.n.min(n);
    |                          --- ^ the trait `Drop` is not implemented for `usize`
    |                          required by a bound introduced by this call
    |
note: required by a bound in `cmp::Ord::min`
   --> library/core/src/cmp.rs:830:15
   --> library/core/src/cmp.rs:830:15
    |
827 |     fn min(self, other: Self) -> Self
    |        --- required by a bound in this
...
830 |         Self: ~const Drop,
    |               ^^^^^^^^^^^ required by this bound in `cmp::Ord::min`
error[E0277]: the trait bound `usize: Drop` is not satisfied
    --> library/core/src/iter/adapters/zip.rs:213:21
     |
     |
213  |         let lower = cmp::min(a_lower, b_lower);
     |                     ^^^^^^^^ the trait `Drop` is not implemented for `usize`
note: required by a bound in `min`
    --> library/core/src/cmp.rs:1214:34
     |
     |
1214 | pub const fn min<T: ~const Ord + ~const Drop>(v1: T, v2: T) -> T {
     |                                  ^^^^^^^^^^^ required by this bound in `min`
error[E0277]: the trait bound `usize: Drop` is not satisfied
    --> library/core/src/iter/adapters/zip.rs:216:40
     |
     |
216  |             (Some(x), Some(y)) => Some(cmp::min(x, y)),
     |                                        ^^^^^^^^ the trait `Drop` is not implemented for `usize`
note: required by a bound in `min`
    --> library/core/src/cmp.rs:1214:34
     |
     |
1214 | pub const fn min<T: ~const Ord + ~const Drop>(v1: T, v2: T) -> T {
     |                                  ^^^^^^^^^^^ required by this bound in `min`
error[E0277]: the trait bound `usize: Drop` is not satisfied
    --> library/core/src/iter/adapters/zip.rs:243:20
     |
     |
243  |         let size = cmp::min(self.a.size(), self.b.size());
     |                    ^^^^^^^^ the trait `Drop` is not implemented for `usize`
note: required by a bound in `min`
    --> library/core/src/cmp.rs:1214:34
     |
     |
1214 | pub const fn min<T: ~const Ord + ~const Drop>(v1: T, v2: T) -> T {
     |                                  ^^^^^^^^^^^ required by this bound in `min`
error[E0277]: the trait bound `usize: Drop` is not satisfied
    --> library/core/src/iter/adapters/zip.rs:264:19
     |
     |
264  |         let len = cmp::min(a_len, b.size());
     |                   ^^^^^^^^ the trait `Drop` is not implemented for `usize`
note: required by a bound in `min`
    --> library/core/src/cmp.rs:1214:34
     |
     |
1214 | pub const fn min<T: ~const Ord + ~const Drop>(v1: T, v2: T) -> T {
     |                                  ^^^^^^^^^^^ required by this bound in `min`
error[E0277]: the trait bound `usize: Drop` is not satisfied
    --> library/core/src/iter/adapters/zip.rs:303:21
     |
     |
303  |         let delta = cmp::min(n, self.len - self.index);
     |                     ^^^^^^^^ the trait `Drop` is not implemented for `usize`
note: required by a bound in `min`
    --> library/core/src/cmp.rs:1214:34
     |
     |
1214 | pub const fn min<T: ~const Ord + ~const Drop>(v1: T, v2: T) -> T {
     |                                  ^^^^^^^^^^^ required by this bound in `min`
error[E0277]: the trait bound `usize: Drop` is not satisfied
   --> library/core/src/iter/range.rs:568:35
    |
    |
568 |         let taken = available.min(n);
    |                               --- ^ the trait `Drop` is not implemented for `usize`
    |                               required by a bound introduced by this call
    |
note: required by a bound in `cmp::Ord::min`
   --> library/core/src/cmp.rs:830:15
   --> library/core/src/cmp.rs:830:15
    |
827 |     fn min(self, other: Self) -> Self
    |        --- required by a bound in this
...
830 |         Self: ~const Drop,
    |               ^^^^^^^^^^^ required by this bound in `cmp::Ord::min`
error[E0277]: the trait bound `usize: Drop` is not satisfied
   --> library/core/src/iter/range.rs:609:35
    |
    |
609 |         let taken = available.min(n);
    |                               --- ^ the trait `Drop` is not implemented for `usize`
    |                               required by a bound introduced by this call
    |
note: required by a bound in `cmp::Ord::min`
   --> library/core/src/cmp.rs:830:15
   --> library/core/src/cmp.rs:830:15
    |
827 |     fn min(self, other: Self) -> Self
    |        --- required by a bound in this
...
830 |         Self: ~const Drop,
    |               ^^^^^^^^^^^ required by this bound in `cmp::Ord::min`
error[E0277]: the trait bound `usize: Drop` is not satisfied
   --> library/core/src/iter/range.rs:652:35
    |
    |
652 |         let taken = available.min(n);
    |                               --- ^ the trait `Drop` is not implemented for `usize`
    |                               required by a bound introduced by this call
    |
note: required by a bound in `cmp::Ord::min`
   --> library/core/src/cmp.rs:830:15
   --> library/core/src/cmp.rs:830:15
    |
827 |     fn min(self, other: Self) -> Self
    |        --- required by a bound in this
...
830 |         Self: ~const Drop,
    |               ^^^^^^^^^^^ required by this bound in `cmp::Ord::min`
error[E0277]: the trait bound `usize: Drop` is not satisfied
   --> library/core/src/iter/range.rs:696:35
    |
    |
696 |         let taken = available.min(n);
    |                               --- ^ the trait `Drop` is not implemented for `usize`
    |                               required by a bound introduced by this call
    |
note: required by a bound in `cmp::Ord::min`
   --> library/core/src/cmp.rs:830:15
   --> library/core/src/cmp.rs:830:15
    |
827 |     fn min(self, other: Self) -> Self
    |        --- required by a bound in this
...
830 |         Self: ~const Drop,
    |               ^^^^^^^^^^^ required by this bound in `cmp::Ord::min`
error[E0277]: the trait bound `usize: Drop` is not satisfied
    --> library/core/src/hash/sip.rs:276:53
     |
     |
276  |             self.tail |= unsafe { u8to64_le(msg, 0, cmp::min(length, needed)) } << (8 * self.ntail);
     |                                                     ^^^^^^^^ the trait `Drop` is not implemented for `usize`
note: required by a bound in `min`
    --> library/core/src/cmp.rs:1214:34
     |
     |
1214 | pub const fn min<T: ~const Ord + ~const Drop>(v1: T, v2: T) -> T {
     |                                  ^^^^^^^^^^^ required by this bound in `min`
error[E0277]: the trait bound `usize: Drop` is not satisfied
    --> library/core/src/slice/memchr.rs:61:18
     |
     |
61   |         offset = cmp::min(offset, len);
     |                  ^^^^^^^^ the trait `Drop` is not implemented for `usize`
note: required by a bound in `min`
    --> library/core/src/cmp.rs:1214:34
     |
     |
1214 | pub const fn min<T: ~const Ord + ~const Drop>(v1: T, v2: T) -> T {
     |                                  ^^^^^^^^^^^ required by this bound in `min`
error[E0277]: the trait bound `usize: Drop` is not satisfied
    --> library/core/src/slice/cmp.rs:104:17
     |
     |
104  |         let l = cmp::min(left.len(), right.len());
     |                 ^^^^^^^^ the trait `Drop` is not implemented for `usize`
note: required by a bound in `min`
    --> library/core/src/cmp.rs:1214:34
     |
     |
1214 | pub const fn min<T: ~const Ord + ~const Drop>(v1: T, v2: T) -> T {
     |                                  ^^^^^^^^^^^ required by this bound in `min`
error[E0277]: the trait bound `usize: Drop` is not satisfied
    --> library/core/src/slice/cmp.rs:168:17
     |
     |
168  |         let l = cmp::min(left.len(), right.len());
     |                 ^^^^^^^^ the trait `Drop` is not implemented for `usize`
note: required by a bound in `min`
    --> library/core/src/cmp.rs:1214:34
     |
     |
1214 | pub const fn min<T: ~const Ord + ~const Drop>(v1: T, v2: T) -> T {
     |                                  ^^^^^^^^^^^ required by this bound in `min`
error[E0277]: the trait bound `usize: Drop` is not satisfied
    --> library/core/src/slice/iter.rs:610:22
     |
     |
610  |             (1, Some(cmp::max(1, self.v.len())))
     |                      ^^^^^^^^ the trait `Drop` is not implemented for `usize`
note: required by a bound in `max`
    --> library/core/src/cmp.rs:1278:34
     |
     |
1278 | pub const fn max<T: ~const Ord + ~const Drop>(v1: T, v2: T) -> T {
     |                                  ^^^^^^^^^^^ required by this bound in `max`
error[E0277]: the trait bound `usize: Drop` is not satisfied
    --> library/core/src/slice/iter.rs:854:22
     |
     |
854  |             (1, Some(cmp::max(1, self.v.len())))
     |                      ^^^^^^^^ the trait `Drop` is not implemented for `usize`
note: required by a bound in `max`
    --> library/core/src/cmp.rs:1278:34
     |
     |
1278 | pub const fn max<T: ~const Ord + ~const Drop>(v1: T, v2: T) -> T {
     |                                  ^^^^^^^^^^^ required by this bound in `max`
error[E0277]: the trait bound `usize: Drop` is not satisfied
    --> library/core/src/slice/iter.rs:1110:13
     |
     |
1110 |             cmp::min(self.count, lower),
     |             ^^^^^^^^ the trait `Drop` is not implemented for `usize`
note: required by a bound in `min`
    --> library/core/src/cmp.rs:1214:34
     |
     |
1214 | pub const fn min<T: ~const Ord + ~const Drop>(v1: T, v2: T) -> T {
     |                                  ^^^^^^^^^^^ required by this bound in `min`
error[E0277]: the trait bound `usize: Drop` is not satisfied
    --> library/core/src/slice/iter.rs:1111:55
     |
     |
1111 |             Some(upper_opt.map_or(self.count, |upper| cmp::min(self.count, upper))),
     |                                                       ^^^^^^^^ the trait `Drop` is not implemented for `usize`
note: required by a bound in `min`
    --> library/core/src/cmp.rs:1214:34
     |
     |
1214 | pub const fn min<T: ~const Ord + ~const Drop>(v1: T, v2: T) -> T {
     |                                  ^^^^^^^^^^^ required by this bound in `min`
error[E0277]: the trait bound `usize: Drop` is not satisfied
    --> library/core/src/slice/iter.rs:1476:27
     |
     |
1476 |             let chunksz = cmp::min(self.v.len(), self.chunk_size);
     |                           ^^^^^^^^ the trait `Drop` is not implemented for `usize`
note: required by a bound in `min`
    --> library/core/src/cmp.rs:1214:34
     |
     |
1214 | pub const fn min<T: ~const Ord + ~const Drop>(v1: T, v2: T) -> T {
     |                                  ^^^^^^^^^^^ required by this bound in `min`
error[E0277]: the trait bound `usize: Drop` is not satisfied
    --> library/core/src/slice/iter.rs:1508:30
     |
     |
1508 |                 Some(sum) => cmp::min(self.v.len(), sum),
     |                              ^^^^^^^^ the trait `Drop` is not implemented for `usize`
note: required by a bound in `min`
    --> library/core/src/cmp.rs:1214:34
     |
     |
1214 | pub const fn min<T: ~const Ord + ~const Drop>(v1: T, v2: T) -> T {
     |                                  ^^^^^^^^^^^ required by this bound in `min`
error[E0277]: the trait bound `usize: Drop` is not satisfied
    --> library/core/src/slice/iter.rs:1537:23
     |
     |
1537 |             let len = cmp::min(self.v.len().unchecked_sub(start), self.chunk_size);
     |                       ^^^^^^^^ the trait `Drop` is not implemented for `usize`
note: required by a bound in `min`
    --> library/core/src/cmp.rs:1214:34
     |
     |
1214 | pub const fn min<T: ~const Ord + ~const Drop>(v1: T, v2: T) -> T {
     |                                  ^^^^^^^^^^^ required by this bound in `min`
error[E0277]: the trait bound `usize: Drop` is not satisfied
    --> library/core/src/slice/iter.rs:1581:30
     |
     |
1581 |                 Some(res) => cmp::min(self.v.len(), res),
     |                              ^^^^^^^^ the trait `Drop` is not implemented for `usize`
note: required by a bound in `min`
    --> library/core/src/cmp.rs:1214:34
     |
     |
1214 | pub const fn min<T: ~const Ord + ~const Drop>(v1: T, v2: T) -> T {
     |                                  ^^^^^^^^^^^ required by this bound in `min`
error[E0277]: the trait bound `usize: Drop` is not satisfied
    --> library/core/src/slice/iter.rs:1651:22
     |
     |
1651 |             let sz = cmp::min(self.v.len(), self.chunk_size);
     |                      ^^^^^^^^ the trait `Drop` is not implemented for `usize`
note: required by a bound in `min`
    --> library/core/src/cmp.rs:1214:34
     |
     |
1214 | pub const fn min<T: ~const Ord + ~const Drop>(v1: T, v2: T) -> T {
     |                                  ^^^^^^^^^^^ required by this bound in `min`
error[E0277]: the trait bound `usize: Drop` is not satisfied
    --> library/core/src/slice/iter.rs:1684:30
     |
     |
1684 |                 Some(sum) => cmp::min(self.v.len(), sum),
     |                              ^^^^^^^^ the trait `Drop` is not implemented for `usize`
note: required by a bound in `min`
    --> library/core/src/cmp.rs:1214:34
     |
     |
1214 | pub const fn min<T: ~const Ord + ~const Drop>(v1: T, v2: T) -> T {
     |                                  ^^^^^^^^^^^ required by this bound in `min`
error[E0277]: the trait bound `usize: Drop` is not satisfied
    --> library/core/src/slice/iter.rs:1714:23
     |
     |
1714 |             let len = cmp::min(self.v.len().unchecked_sub(start), self.chunk_size);
     |                       ^^^^^^^^ the trait `Drop` is not implemented for `usize`
note: required by a bound in `min`
    --> library/core/src/cmp.rs:1214:34
     |
     |
1214 | pub const fn min<T: ~const Ord + ~const Drop>(v1: T, v2: T) -> T {
     |                                  ^^^^^^^^^^^ required by this bound in `min`
error[E0277]: the trait bound `usize: Drop` is not satisfied
    --> library/core/src/slice/iter.rs:1747:30
     |
     |
1747 |                 Some(res) => cmp::min(self.v.len(), res),
     |                              ^^^^^^^^ the trait `Drop` is not implemented for `usize`
note: required by a bound in `min`
    --> library/core/src/cmp.rs:1214:34
     |
     |
1214 | pub const fn min<T: ~const Ord + ~const Drop>(v1: T, v2: T) -> T {
     |                                  ^^^^^^^^^^^ required by this bound in `min`
error[E0277]: the trait bound `usize: Drop` is not satisfied
    --> library/core/src/slice/iter.rs:2504:27
     |
     |
2504 |             let chunksz = cmp::min(len, self.chunk_size);
     |                           ^^^^^^^^ the trait `Drop` is not implemented for `usize`
note: required by a bound in `min`
    --> library/core/src/cmp.rs:1214:34
     |
     |
1214 | pub const fn min<T: ~const Ord + ~const Drop>(v1: T, v2: T) -> T {
     |                                  ^^^^^^^^^^^ required by this bound in `min`
error[E0277]: the trait bound `usize: Drop` is not satisfied
    --> library/core/src/slice/iter.rs:2668:22
     |
     |
2668 |             let sz = cmp::min(self.v.len(), self.chunk_size);
     |                      ^^^^^^^^ the trait `Drop` is not implemented for `usize`
note: required by a bound in `min`
    --> library/core/src/cmp.rs:1214:34
     |
     |
1214 | pub const fn min<T: ~const Ord + ~const Drop>(v1: T, v2: T) -> T {
     |                                  ^^^^^^^^^^^ required by this bound in `min`
error[E0277]: the trait bound `usize: Drop` is not satisfied
    --> library/core/src/slice/rotate.rs:161:19
     |
     |
161  |         } else if cmp::min(left, right) <= mem::size_of::<BufType>() / mem::size_of::<T>() {
     |                   ^^^^^^^^ the trait `Drop` is not implemented for `usize`
note: required by a bound in `min`
    --> library/core/src/cmp.rs:1214:34
     |
     |
1214 | pub const fn min<T: ~const Ord + ~const Drop>(v1: T, v2: T) -> T {
     |                                  ^^^^^^^^^^^ required by this bound in `min`
error[E0277]: the trait bound `usize: Drop` is not satisfied
    --> library/core/src/slice/sort.rs:363:21
     |
     |
363  |         let count = cmp::min(width(start_l, end_l), width(start_r, end_r));
     |                     ^^^^^^^^ the trait `Drop` is not implemented for `usize`
note: required by a bound in `min`
    --> library/core/src/cmp.rs:1214:34
     |
     |
1214 | pub const fn min<T: ~const Ord + ~const Drop>(v1: T, v2: T) -> T {
     |                                  ^^^^^^^^^^^ required by this bound in `min`
error[E0277]: the trait bound `usize: Drop` is not satisfied
    --> library/core/src/slice/sort.rs:788:24
     |
     |
788  |         was_balanced = cmp::min(mid, len - mid) >= len / 8;
     |                        ^^^^^^^^ the trait `Drop` is not implemented for `usize`
note: required by a bound in `min`
    --> library/core/src/cmp.rs:1214:34
     |
     |
1214 | pub const fn min<T: ~const Ord + ~const Drop>(v1: T, v2: T) -> T {
     |                                  ^^^^^^^^^^^ required by this bound in `min`
error[E0277]: the trait bound `usize: Drop` is not satisfied
    --> library/core/src/slice/mod.rs:3382:31
     |
     |
3382 |             let k = ctz_a.min(ctz_b);
     |                           --- ^^^^^ the trait `Drop` is not implemented for `usize`
     |                           required by a bound introduced by this call
     |
note: required by a bound in `cmp::Ord::min`
    --> library/core/src/cmp.rs:830:15
    --> library/core/src/cmp.rs:830:15
     |
827  |     fn min(self, other: Self) -> Self
     |        --- required by a bound in this
...
830  |         Self: ~const Drop,
