plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: this stability annotation is useless
    --> library/core/src/mem/maybe_uninit.rs:1167:5
     |
1167 |       #[unstable(feature = "maybe_uninit_array_index", issue = "none")]
     |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ useless stability annotation
1168 |       #[inline]
1169 | /     fn index(&self, index: usize) -> &Self::Output {
1170 | |         Index::index(self.as_ref(), index)
     | |_____- the stability attribute annotates this item

error: this stability annotation is useless
    --> library/core/src/mem/maybe_uninit.rs:1175:5
    --> library/core/src/mem/maybe_uninit.rs:1175:5
     |
1175 |       #[unstable(feature = "maybe_uninit_array_index", issue = "none")]
     |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ useless stability annotation
1177 | /     fn index_mut(&mut self, index: usize) -> &mut Self::Output {
1178 | |         IndexMut::index_mut(self.as_mut(), index)
1179 | |     }
     | |_____- the stability attribute annotates this item
     | |_____- the stability attribute annotates this item

error: this stability annotation is useless
    --> library/core/src/mem/maybe_uninit.rs:1193:5
     |
1193 |       #[unstable(feature = "maybe_uninit_array_index", issue = "none")]
     |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ useless stability annotation
1194 |       #[inline]
1195 | /     fn as_mut(&mut self) -> &mut [MaybeUninit<T>] {
1196 | |         let data = self.as_mut_ptr().cast::<MaybeUninit<T>>();
1197 | |         // SAFETY: MaybeUninit<[T; N]> and [MaybeUninit<T>; N] have the same layout
1198 | |         // data points to N consecutive properly initialized values of type MaybeUninit<T>.
1199 | |         unsafe { crate::slice::from_raw_parts_mut(data, N) }
     | |_____- the stability attribute annotates this item

error: this stability annotation is useless
    --> library/core/src/mem/maybe_uninit.rs:1207:5
    --> library/core/src/mem/maybe_uninit.rs:1207:5
     |
1207 |       #[unstable(feature = "maybe_uninit_array_index", issue = "none")]
     |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ useless stability annotation
1208 | /     fn into_iter(self) -> IterMut<'a, MaybeUninit<T>> {
1209 | |         self.as_mut().iter_mut()
     | |_____- the stability attribute annotates this item

error: could not compile `core` due to 4 previous errors
Build completed unsuccessfully in 0:01:14
