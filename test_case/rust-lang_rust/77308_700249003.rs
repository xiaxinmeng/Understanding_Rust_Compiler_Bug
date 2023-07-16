plain
Initialized empty Git repository in /home/runner/work/rust/rust/.git/
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/77308/merge:refs/remotes/pull/77308/merge
---
   Compiling libc v0.2.74
   Compiling autocfg v1.0.0
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling hashbrown v0.8.2
error[E0407]: method `get_unchecked` is not a member of trait `Iterator`
     |
     |
3645 |  / macro_rules! iterator {
3646 |  |     (
3647 |  |         struct $name:ident -> $ptr:ty,
3648 |  |         $elem:ty,
...     |
3925 | /|             unsafe fn get_unchecked(&mut self, idx: usize) -> Self::Item {
3926 | ||                 // SAFETY: the caller must guarantee that `i` is in bounds of
3927 | ||                 // the underlying slice, so `i` cannot overflow an `isize`, and
3928 | ||                 // the returned references is guaranteed to refer to an element
...    ||
3936 | ||                 unsafe { & $( $mut_ )? * self.ptr.as_ptr().add(idx) }
3937 | ||             }
     | ||_____________^ not a member of trait `Iterator`
3985 |  |     }
3986 |  | }
3986 |  | }
     |  |_- in this expansion of `iterator!`
...
4060 | /  iterator! {struct Iter -> *const T, &'a T, const, {/* no mut */}, {
4061 | |      fn is_sorted_by<F>(self, mut compare: F) -> bool
4062 | |      where
4063 | |          Self: Sized,
4069 | |      }
4070 | |  }}
     | |___- in this macro invocation


error[E0407]: method `get_unchecked` is not a member of trait `Iterator`
     |
     |
3645 |  / macro_rules! iterator {
3646 |  |     (
3647 |  |         struct $name:ident -> $ptr:ty,
3648 |  |         $elem:ty,
...     |
3925 | /|             unsafe fn get_unchecked(&mut self, idx: usize) -> Self::Item {
3926 | ||                 // SAFETY: the caller must guarantee that `i` is in bounds of
3927 | ||                 // the underlying slice, so `i` cannot overflow an `isize`, and
3928 | ||                 // the returned references is guaranteed to refer to an element
...    ||
3936 | ||                 unsafe { & $( $mut_ )? * self.ptr.as_ptr().add(idx) }
3937 | ||             }
     | ||_____________^ not a member of trait `Iterator`
3985 |  |     }
3986 |  | }
3986 |  | }
     |  |_- in this expansion of `iterator!`
...
4202 |    iterator! {struct IterMut -> *mut T, &'a mut T, mut, {mut}, {}}


error[E0407]: method `get_unchecked` is not a member of trait `Iterator`
     |
     |
5025 | /     unsafe fn get_unchecked(&mut self, idx: usize) -> Self::Item {
5026 | |         // SAFETY: since the caller guarantees that `i` is in bounds,
5027 | |         // which means that `i` cannot overflow an `isize`, and the
5028 | |         // slice created by `from_raw_parts` is a subslice of `self.v`
5029 | |         // thus is guaranteed to be valid for the lifetime `'a` of `self.v`.
5030 | |         unsafe { from_raw_parts(self.v.as_ptr().add(idx), self.size) }
5031 | |     }
     | |_____^ not a member of trait `Iterator`

error[E0407]: method `get_unchecked` is not a member of trait `Iterator`
     |
     |
5164 | /     unsafe fn get_unchecked(&mut self, idx: usize) -> Self::Item {
5165 | |         let start = idx * self.chunk_size;
5166 | |         let end = match start.checked_add(self.chunk_size) {
5167 | |             None => self.v.len(),
...    |
5177 | |         unsafe { from_raw_parts(self.v.as_ptr().add(start), end - start) }
5178 | |     }
     | |_____^ not a member of trait `Iterator`

error[E0407]: method `get_unchecked` is not a member of trait `Iterator`
     |
     |
5313 | /     unsafe fn get_unchecked(&mut self, idx: usize) -> Self::Item {
5314 | |         let start = idx * self.chunk_size;
5315 | |         let end = match start.checked_add(self.chunk_size) {
5316 | |             None => self.v.len(),
...    |
5325 | |         unsafe { from_raw_parts_mut(self.v.as_mut_ptr().add(start), end - start) }
5326 | |     }
     | |_____^ not a member of trait `Iterator`

error[E0407]: method `get_unchecked` is not a member of trait `Iterator`
     |
     |
5466 | /     unsafe fn get_unchecked(&mut self, idx: usize) -> Self::Item {
5467 | |         let start = idx * self.chunk_size;
5468 | |         // SAFETY: mostly identical to `Chunks::get_unchecked`.
5469 | |         unsafe { from_raw_parts(self.v.as_ptr().add(start), self.chunk_size) }
5470 | |     }
     | |_____^ not a member of trait `Iterator`

error[E0407]: method `get_unchecked` is not a member of trait `Iterator`
     |
     |
5600 | /     unsafe fn get_unchecked(&mut self, idx: usize) -> Self::Item {
5601 | |         let start = idx * self.chunk_size;
5602 | |         // SAFETY: see comments for `ChunksMut::get_unchecked`.
5603 | |         unsafe { from_raw_parts_mut(self.v.as_mut_ptr().add(start), self.chunk_size) }
5604 | |     }
     | |_____^ not a member of trait `Iterator`

error[E0407]: method `get_unchecked` is not a member of trait `Iterator`
     |
     |
5726 | /     unsafe fn get_unchecked(&mut self, i: usize) -> &'a [T; N] {
5727 | |         // SAFETY: The safety guarantees of `get_unchecked` are transferred to
5728 | |         // the caller.
5729 | |         unsafe { self.iter.get_unchecked(i) }
5730 | |     }
     | |_____^ not a member of trait `Iterator`

error[E0407]: method `get_unchecked` is not a member of trait `Iterator`
     |
     |
5856 | /     unsafe fn get_unchecked(&mut self, idx: usize) -> Self::Item {
5857 | |         let end = self.v.len() - idx * self.chunk_size;
5858 | |         let start = match end.checked_sub(self.chunk_size) {
5859 | |             None => 0,
...    |
5863 | |         unsafe { from_raw_parts(self.v.as_ptr().add(start), end - start) }
5864 | |     }
     | |_____^ not a member of trait `Iterator`

error[E0407]: method `get_unchecked` is not a member of trait `Iterator`
     |
     |
6002 | /     unsafe fn get_unchecked(&mut self, idx: usize) -> Self::Item {
6003 | |         let end = self.v.len() - idx * self.chunk_size;
6004 | |         let start = match end.checked_sub(self.chunk_size) {
6005 | |             None => 0,
...    |
6009 | |         unsafe { from_raw_parts_mut(self.v.as_mut_ptr().add(start), end - start) }
6010 | |     }
     | |_____^ not a member of trait `Iterator`

error[E0407]: method `get_unchecked` is not a member of trait `Iterator`
     |
     |
6148 | /     unsafe fn get_unchecked(&mut self, idx: usize) -> Self::Item {
6149 | |         let end = self.v.len() - idx * self.chunk_size;
6150 | |         let start = end - self.chunk_size;
6151 | |         // SAFETY:
6152 | |         // SAFETY: mostmy identical to `Chunks::get_unchecked`.
6153 | |         unsafe { from_raw_parts(self.v.as_ptr().add(start), self.chunk_size) }
6154 | |     }
     | |_____^ not a member of trait `Iterator`

error[E0407]: method `get_unchecked` is not a member of trait `Iterator`
     |
     |
6289 | /     unsafe fn get_unchecked(&mut self, idx: usize) -> Self::Item {
6290 | |         let end = self.v.len() - idx * self.chunk_size;
6291 | |         let start = end - self.chunk_size;
6292 | |         // SAFETY: see comments for `RChunksMut::get_unchecked`.
6293 | |         unsafe { from_raw_parts_mut(self.v.as_mut_ptr().add(start), self.chunk_size) }
6294 | |     }
     | |_____^ not a member of trait `Iterator`

error[E0599]: no method named `get_unchecked` found for type parameter `A` in the current scope
   --> library/core/src/iter/adapters/zip.rs:200:35
    |
200 |             unsafe { Some((self.a.get_unchecked(i), self.b.get_unchecked(i))) }
    |                                   ^^^^^^^^^^^^^ method not found in `A`
    |
    = help: items from traits can only be used if the type parameter is bounded by the trait
help: the following traits define an item `get_unchecked`, perhaps you need to restrict type parameter `A` with one of them:
    |
184 | impl<A: iter::adapters::zip::ZipImpl, B> ZipImpl<A, B> for Zip<A, B>
    |      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
184 | impl<A: slice::SliceIndex, B> ZipImpl<A, B> for Zip<A, B>


error[E0599]: no method named `get_unchecked` found for type parameter `B` in the current scope
   --> library/core/src/iter/adapters/zip.rs:200:60
    |
200 |             unsafe { Some((self.a.get_unchecked(i), self.b.get_unchecked(i))) }
    |                                                            ^^^^^^^^^^^^^ method not found in `B`
    |
    = help: items from traits can only be used if the type parameter is bounded by the trait
help: the following traits define an item `get_unchecked`, perhaps you need to restrict type parameter `B` with one of them:
    |
184 | impl<A, B: iter::adapters::zip::ZipImpl> ZipImpl<A, B> for Zip<A, B>
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
184 | impl<A, B: slice::SliceIndex> ZipImpl<A, B> for Zip<A, B>


error[E0599]: no method named `get_unchecked` found for type parameter `A` in the current scope
   --> library/core/src/iter/adapters/zip.rs:205:24
    |
205 |                 self.a.get_unchecked(self.index);
    |                        ^^^^^^^^^^^^^ method not found in `A`
    |
    = help: items from traits can only be used if the type parameter is bounded by the trait
help: the following traits define an item `get_unchecked`, perhaps you need to restrict type parameter `A` with one of them:
    |
184 | impl<A: iter::adapters::zip::ZipImpl, B> ZipImpl<A, B> for Zip<A, B>
    |      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
184 | impl<A: slice::SliceIndex, B> ZipImpl<A, B> for Zip<A, B>


error[E0599]: no method named `get_unchecked` found for type parameter `A` in the current scope
   --> library/core/src/iter/adapters/zip.rs:232:28
    |
232 |                     self.a.get_unchecked(i);
    |                            ^^^^^^^^^^^^^ method not found in `A`
    |
    = help: items from traits can only be used if the type parameter is bounded by the trait
help: the following traits define an item `get_unchecked`, perhaps you need to restrict type parameter `A` with one of them:
    |
184 | impl<A: iter::adapters::zip::ZipImpl, B> ZipImpl<A, B> for Zip<A, B>
    |      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
184 | impl<A: slice::SliceIndex, B> ZipImpl<A, B> for Zip<A, B>


error[E0599]: no method named `get_unchecked` found for type parameter `B` in the current scope
   --> library/core/src/iter/adapters/zip.rs:238:28
    |
238 |                     self.b.get_unchecked(i);
    |                            ^^^^^^^^^^^^^ method not found in `B`
    |
    = help: items from traits can only be used if the type parameter is bounded by the trait
help: the following traits define an item `get_unchecked`, perhaps you need to restrict type parameter `B` with one of them:
    |
184 | impl<A, B: iter::adapters::zip::ZipImpl> ZipImpl<A, B> for Zip<A, B>
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
184 | impl<A, B: slice::SliceIndex> ZipImpl<A, B> for Zip<A, B>


error[E0599]: no method named `get_unchecked` found for type parameter `A` in the current scope
   --> library/core/src/iter/adapters/zip.rs:280:35
    |
280 |             unsafe { Some((self.a.get_unchecked(i), self.b.get_unchecked(i))) }
    |                                   ^^^^^^^^^^^^^ method not found in `A`
    |
    = help: items from traits can only be used if the type parameter is bounded by the trait
help: the following traits define an item `get_unchecked`, perhaps you need to restrict type parameter `A` with one of them:
    |
184 | impl<A: iter::adapters::zip::ZipImpl, B> ZipImpl<A, B> for Zip<A, B>
    |      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
184 | impl<A: slice::SliceIndex, B> ZipImpl<A, B> for Zip<A, B>


error[E0599]: no method named `get_unchecked` found for type parameter `B` in the current scope
   --> library/core/src/iter/adapters/zip.rs:280:60
    |
280 |             unsafe { Some((self.a.get_unchecked(i), self.b.get_unchecked(i))) }
    |                                                            ^^^^^^^^^^^^^ method not found in `B`
    |
    = help: items from traits can only be used if the type parameter is bounded by the trait
help: the following traits define an item `get_unchecked`, perhaps you need to restrict type parameter `B` with one of them:
    |
184 | impl<A, B: iter::adapters::zip::ZipImpl> ZipImpl<A, B> for Zip<A, B>
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
184 | impl<A, B: slice::SliceIndex> ZipImpl<A, B> for Zip<A, B>


error[E0599]: no method named `get_unchecked` found for type parameter `A` in the current scope
   --> library/core/src/iter/adapters/zip.rs:290:26
    |
290 |         unsafe { (self.a.get_unchecked(idx), self.b.get_unchecked(idx)) }
    |                          ^^^^^^^^^^^^^ method not found in `A`
    |
    = help: items from traits can only be used if the type parameter is bounded by the trait
help: the following traits define an item `get_unchecked`, perhaps you need to restrict type parameter `A` with one of them:
    |
184 | impl<A: iter::adapters::zip::ZipImpl, B> ZipImpl<A, B> for Zip<A, B>
    |      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
184 | impl<A: slice::SliceIndex, B> ZipImpl<A, B> for Zip<A, B>


error[E0599]: no method named `get_unchecked` found for type parameter `B` in the current scope
   --> library/core/src/iter/adapters/zip.rs:290:53
    |
290 |         unsafe { (self.a.get_unchecked(idx), self.b.get_unchecked(idx)) }
    |                                                     ^^^^^^^^^^^^^ method not found in `B`
    |
    = help: items from traits can only be used if the type parameter is bounded by the trait
help: the following traits define an item `get_unchecked`, perhaps you need to restrict type parameter `B` with one of them:
    |
184 | impl<A, B: iter::adapters::zip::ZipImpl> ZipImpl<A, B> for Zip<A, B>
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
184 | impl<A, B: slice::SliceIndex> ZipImpl<A, B> for Zip<A, B>


error[E0599]: no method named `get_unchecked` found for mutable reference `&mut I` in the current scope
   --> library/core/src/iter/adapters/zip.rs:433:23
    |
433 |         unsafe { self.get_unchecked(index) }
    |                       ^^^^^^^^^^^^^ method not found in `&mut I`
    |
    = help: items from traits can only be used if the type parameter is bounded by the trait
help: the following traits define an item `get_unchecked`, perhaps you need to restrict type parameter `I` with one of them:
    |
429 | unsafe impl<I: iter::adapters::zip::ZipImpl + Iterator + TrustedRandomAccess> SpecTrustedRandomAccess for I {
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
429 | unsafe impl<I: slice::SliceIndex + Iterator + TrustedRandomAccess> SpecTrustedRandomAccess for I {

   Compiling compiler_builtins v0.1.32
   Compiling unwind v0.0.0 (/checkout/library/unwind)
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0599]: no method named `get_unchecked` found for struct `slice::Iter<'a, [T; N]>` in the current scope
     |
     |
4009 | pub struct Iter<'a, T: 'a> {
     | -------------------------- method `get_unchecked` not found for this
...
5729 |         unsafe { self.iter.get_unchecked(i) }
     |                            ^^^^^^^^^^^^^ method not found in `slice::Iter<'a, [T; N]>`
     |
     = help: items from traits can only be used if the trait is implemented and in scope
     = note: the following traits define an item `get_unchecked`, perhaps you need to implement one of them:
             candidate #1: `iter::adapters::zip::ZipImpl`
             candidate #2: `slice::SliceIndex`

error[E0599]: no method named `get_unchecked` found for struct `iter::adapters::Copied<slice::Iter<'_, u8>>` in the current scope
    |
    |
830 |         unsafe { self.0.get_unchecked(idx) }
    |                         ^^^^^^^^^^^^^ method not found in `iter::adapters::Copied<slice::Iter<'_, u8>>`
   ::: library/core/src/iter/adapters/mod.rs:156:1
    |
    |
156 | pub struct Copied<I> {
    | -------------------- method `get_unchecked` not found for this
    |
    = help: items from traits can only be used if the trait is implemented and in scope
    = note: the following traits define an item `get_unchecked`, perhaps you need to implement one of them:
            candidate #1: `iter::adapters::zip::ZipImpl`
            candidate #2: `slice::SliceIndex`
error: aborting due to 24 previous errors

Some errors have detailed explanations: E0407, E0599.
For more information about an error, try `rustc --explain E0407`.
---
== clock drift check ==
  local time: Mon Sep 28 19:57:21 UTC 2020
  network time: Mon, 28 Sep 2020 19:57:21 GMT
== end clock drift check ==
##[error]Process completed with exit code 1.
Terminate orphan process: pid (5191) (node)
Terminate orphan process: pid (5248) (python)
