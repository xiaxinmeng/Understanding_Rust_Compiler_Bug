
error: trait has missing stability attribute
  --> library/core/src/ptr/metadata.rs:51:1
   |
51 | / pub trait Pointee {
52 | |     /// The type for metadata in pointers and references to `Self`.
53 | |     #[lang = "metadata_type"]
54 | |     // NOTE: Keep trait bounds in `static_assert_expected_bounds_for_metadata`
...  |
57 | |     type Metadata: Copy + Send + Sync + Ord + Hash + Unpin;
58 | | }
   | |_^

error: function has missing stability attribute
  --> library/core/src/ptr/metadata.rs:91:1
   |
91 | / pub const fn metadata<T: ?Sized>(ptr: *const T) -> <T as Pointee>::Metadata {
92 | |     // SAFETY: Accessing the value from the `PtrRepr` union is safe since *const T
93 | |     // and PtrComponents<T> have the same memory layouts. Only std can make this
94 | |     // guarantee.
95 | |     unsafe { PtrRepr { const_ptr: ptr }.components.metadata }
96 | | }
   | |_^

error: struct has missing stability attribute
   --> library/core/src/ptr/metadata.rs:176:1
    |
176 | / pub struct DynMetadata<Dyn: ?Sized> {
177 | |     vtable_ptr: &'static VTable,
178 | |     phantom: crate::marker::PhantomData<Dyn>,
179 | | }
    | |_^

error: implementation has missing stability attribute
   --> library/core/src/ptr/metadata.rs:213:1
    |
213 | unsafe impl<Dyn: ?Sized> Send for DynMetadata<Dyn> {}
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: implementation has missing stability attribute
   --> library/core/src/ptr/metadata.rs:214:1
    |
214 | unsafe impl<Dyn: ?Sized> Sync for DynMetadata<Dyn> {}
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: implementation has missing stability attribute
   --> library/core/src/ptr/metadata.rs:216:1
    |
216 | / impl<Dyn: ?Sized> fmt::Debug for DynMetadata<Dyn> {
217 | |     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
218 | |         f.debug_tuple("DynMetadata").field(&(self.vtable_ptr as *const VTable)).finish()
219 | |     }
220 | | }
    | |_^

error: implementation has missing stability attribute
   --> library/core/src/ptr/metadata.rs:224:1
    |
224 | impl<Dyn: ?Sized> Unpin for DynMetadata<Dyn> {}
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: implementation has missing stability attribute
   --> library/core/src/ptr/metadata.rs:226:1
    |
226 | impl<Dyn: ?Sized> Copy for DynMetadata<Dyn> {}
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: implementation has missing stability attribute
   --> library/core/src/ptr/metadata.rs:228:1
    |
228 | / impl<Dyn: ?Sized> Clone for DynMetadata<Dyn> {
229 | |     #[inline]
230 | |     fn clone(&self) -> Self {
231 | |         *self
232 | |     }
233 | | }
    | |_^

error: implementation has missing stability attribute
   --> library/core/src/ptr/metadata.rs:235:1
    |
235 | impl<Dyn: ?Sized> Eq for DynMetadata<Dyn> {}
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: implementation has missing stability attribute
   --> library/core/src/ptr/metadata.rs:237:1
    |
237 | / impl<Dyn: ?Sized> PartialEq for DynMetadata<Dyn> {
238 | |     #[inline]
239 | |     fn eq(&self, other: &Self) -> bool {
240 | |         crate::ptr::eq::<VTable>(self.vtable_ptr, other.vtable_ptr)
241 | |     }
242 | | }
    | |_^

error: implementation has missing stability attribute
   --> library/core/src/ptr/metadata.rs:244:1
    |
244 | / impl<Dyn: ?Sized> Ord for DynMetadata<Dyn> {
245 | |     #[inline]
246 | |     fn cmp(&self, other: &Self) -> crate::cmp::Ordering {
247 | |         (self.vtable_ptr as *const VTable).cmp(&(other.vtable_ptr as *const VTable))
248 | |     }
249 | | }
    | |_^

error: implementation has missing stability attribute
   --> library/core/src/ptr/metadata.rs:251:1
    |
251 | / impl<Dyn: ?Sized> PartialOrd for DynMetadata<Dyn> {
252 | |     #[inline]
253 | |     fn partial_cmp(&self, other: &Self) -> Option<crate::cmp::Ordering> {
254 | |         Some(self.cmp(other))
255 | |     }
256 | | }
    | |_^
