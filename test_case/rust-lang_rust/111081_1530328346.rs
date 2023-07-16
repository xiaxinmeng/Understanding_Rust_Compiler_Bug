plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0433]: failed to resolve: use of undeclared type `Bound`
   --> library/core/src/str/traits.rs:260:9
    |
260 |         Bound::Included(included) => included,

error[E0433]: failed to resolve: use of undeclared type `Bound`
   --> library/core/src/str/traits.rs:261:9
    |
    |
261 |         Bound::Excluded(excluded) => excluded.checked_add(1)?,

error[E0433]: failed to resolve: use of undeclared type `Bound`
   --> library/core/src/str/traits.rs:262:9
    |
    |
262 |         Bound::Unbounded => 0,
    |         ^^^^^ use of undeclared type `Bound`

error[E0433]: failed to resolve: use of undeclared type `Bound`
   --> library/core/src/str/traits.rs:265:9
    |
265 |         Bound::Included(included) => included.checked_add(1)?,

error[E0433]: failed to resolve: use of undeclared type `Bound`
   --> library/core/src/str/traits.rs:266:9
    |
    |
266 |         Bound::Excluded(excluded) => excluded,

error[E0433]: failed to resolve: use of undeclared type `Bound`
   --> library/core/src/str/traits.rs:267:9
    |
    |
267 |         Bound::Unbounded => slice.len(),

error[E0412]: cannot find type `Bound` in this scope
   --> library/core/src/str/traits.rs:257:20
    |
    |
257 |     (start, end): (Bound<usize>, Bound<usize>),
    |
help: consider importing this enum
    |
3   + use crate::ops::Bound;
3   + use crate::ops::Bound;
    |

error[E0412]: cannot find type `Bound` in this scope
   --> library/core/src/str/traits.rs:257:34
    |
257 |     (start, end): (Bound<usize>, Bound<usize>),
    |
help: consider importing this enum
    |
3   + use crate::ops::Bound;
---

error[E0425]: cannot find value `slice` in this scope
   --> library/core/src/str/traits.rs:267:29
    |
267 |         Bound::Unbounded => slice.len(),

error[E0412]: cannot find type `Bound` in this scope
   --> library/core/src/str/traits.rs:285:34
    |
    |
285 | unsafe impl SliceIndex<str> for (Bound<usize>, Bound<usize>) {
    |
help: consider importing this enum
    |
3   + use crate::ops::Bound;
3   + use crate::ops::Bound;
    |

error[E0412]: cannot find type `Bound` in this scope
   --> library/core/src/str/traits.rs:285:48
    |
285 | unsafe impl SliceIndex<str> for (Bound<usize>, Bound<usize>) {
    |
help: consider importing this enum
    |
3   + use crate::ops::Bound;
3   + use crate::ops::Bound;
    |

error: implementation has missing stability attribute
   --> library/core/src/str/traits.rs:285:1
    |
285 | / unsafe impl SliceIndex<str> for (Bound<usize>, Bound<usize>) {
286 | |     type Output = str;
287 | |     #[inline]
288 | |     fn get(self, slice: &str) -> Option<&Self::Output> {
310 | |     }
311 | | }
    | |_^


error[E0061]: this function takes 2 arguments but 1 argument was supplied
   --> library/core/src/str/traits.rs:289:9
    |
289 |         bounds_to_range(self)?.get(slice)
    |
note: function defined here
   --> library/core/src/str/traits.rs:255:4
    |
    |
255 | fn bounds_to_range(
    |    ^^^^^^^^^^^^^^^
256 |     input: &str,
    |     -----------
257 |     (start, end): (Bound<usize>, Bound<usize>),
help: provide the argument
    |
    |
289 |         bounds_to_range(self, /*  */)?.get(slice)

error[E0061]: this function takes 2 arguments but 1 argument was supplied
   --> library/core/src/str/traits.rs:293:9
    |
    |
293 |         bounds_to_range(self)?.get_mut(slice)
    |
note: function defined here
   --> library/core/src/str/traits.rs:255:4
    |
    |
255 | fn bounds_to_range(
    |    ^^^^^^^^^^^^^^^
256 |     input: &str,
    |     -----------
257 |     (start, end): (Bound<usize>, Bound<usize>),
help: provide the argument
    |
    |
293 |         bounds_to_range(self, /*  */)?.get_mut(slice)

error[E0061]: this function takes 2 arguments but 1 argument was supplied
   --> library/core/src/str/traits.rs:297:9
    |
    |
297 |         bounds_to_range(self)?.get_unchecked(slice)
    |
note: function defined here
   --> library/core/src/str/traits.rs:255:4
    |
    |
255 | fn bounds_to_range(
    |    ^^^^^^^^^^^^^^^
256 |     input: &str,
    |     -----------
257 |     (start, end): (Bound<usize>, Bound<usize>),
help: provide the argument
    |
    |
297 |         bounds_to_range(self, /*  */)?.get_unchecked(slice)

error[E0061]: this function takes 2 arguments but 1 argument was supplied
   --> library/core/src/str/traits.rs:301:9
    |
    |
301 |         bounds_to_range(self)?.get_unchecked_mut(slice)
    |
note: function defined here
   --> library/core/src/str/traits.rs:255:4
    |
    |
255 | fn bounds_to_range(
    |    ^^^^^^^^^^^^^^^
256 |     input: &str,
    |     -----------
257 |     (start, end): (Bound<usize>, Bound<usize>),
help: provide the argument
    |
    |
301 |         bounds_to_range(self, /*  */)?.get_unchecked_mut(slice)

error[E0061]: this function takes 2 arguments but 1 argument was supplied
   --> library/core/src/str/traits.rs:305:9
    |
    |
305 |         bounds_to_range(self)?.index(slice)
    |
note: function defined here
   --> library/core/src/str/traits.rs:255:4
    |
    |
255 | fn bounds_to_range(
    |    ^^^^^^^^^^^^^^^
256 |     input: &str,
    |     -----------
257 |     (start, end): (Bound<usize>, Bound<usize>),
help: provide the argument
    |
    |
305 |         bounds_to_range(self, /*  */)?.index(slice)

error[E0061]: this function takes 2 arguments but 1 argument was supplied
   --> library/core/src/str/traits.rs:309:9
    |
    |
309 |         bounds_to_range(self)?.index_mut(slice)
    |
note: function defined here
   --> library/core/src/str/traits.rs:255:4
    |
    |
255 | fn bounds_to_range(
    |    ^^^^^^^^^^^^^^^
256 |     input: &str,
    |     -----------
257 |     (start, end): (Bound<usize>, Bound<usize>),
help: provide the argument
    |
    |
309 |         bounds_to_range(self, /*  */)?.index_mut(slice)

Some errors have detailed explanations: E0061, E0412, E0425, E0433.
For more information about an error, try `rustc --explain E0061`.
error: could not compile `core` (lib) due to 19 previous errors
