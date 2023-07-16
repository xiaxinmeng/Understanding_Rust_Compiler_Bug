plain
   Compiling alloc v0.0.0 (/checkout/library/alloc)
   Compiling cfg-if v0.1.10
   Compiling adler v0.2.3
   Compiling rustc-demangle v0.1.21
error[E0520]: `from_elem` specializes an item from a parent `impl`, but that item is not marked `default`
   |
   |
21 | / impl<T: Clone + IsZero> SpecFromElem for T {
22 | |     #[inline]
23 | |     fn from_elem<A: Allocator>(elem: T, n: usize, alloc: A) -> Vec<T, A> {
24 | |         if elem.is_zero() {
30 | |     }
31 | | }
31 | | }
   | |_- parent `impl` is here
...
35 | /     fn from_elem<A: Allocator>(elem: i8, n: usize, alloc: A) -> Vec<i8, A> {
36 | |         if elem == 0 {
37 | |             return Vec { buf: Box::new_zeroed_slice_in(n, alloc), phantom: PhantomData, len: n };
...  |
44 | |         }
45 | |     }
45 | |     }
   | |_____^ cannot specialize default item `from_elem`
   |
   = note: to specialize, `from_elem` in the parent `impl` must be marked `default`

error[E0520]: `from_elem` specializes an item from a parent `impl`, but that item is not marked `default`
   |
   |
21 | / impl<T: Clone + IsZero> SpecFromElem for T {
22 | |     #[inline]
23 | |     fn from_elem<A: Allocator>(elem: T, n: usize, alloc: A) -> Vec<T, A> {
24 | |         if elem.is_zero() {
30 | |     }
31 | | }
31 | | }
   | |_- parent `impl` is here
...
50 | /     fn from_elem<A: Allocator>(elem: u8, n: usize, alloc: A) -> Vec<u8, A> {
51 | |         if elem == 0 {
52 | |             return Vec { buf: Box::new_zeroed_slice_in(n, alloc), phantom: PhantomData, len: n };
...  |
59 | |         }
60 | |     }
60 | |     }
   | |_____^ cannot specialize default item `from_elem`
   |
   = note: to specialize, `from_elem` in the parent `impl` must be marked `default`
For more information about this error, try `rustc --explain E0520`.
error: could not compile `alloc` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:00:18
