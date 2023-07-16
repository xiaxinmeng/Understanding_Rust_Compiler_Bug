plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: implementation has missing stability attribute
    --> library/core/src/mem/maybe_uninit.rs:1164:1
     |
1164 | / impl<T, const N: usize> Index<usize> for MaybeUninit<[T; N]> {
1165 | |     type Output = MaybeUninit<T>;
1167 | |     #[inline]
...    |
1170 | |     }
1171 | | }
1171 | | }
     | |_^

error: implementation has missing stability attribute
    --> library/core/src/mem/maybe_uninit.rs:1173:1
     |
1173 | / impl<T, const N: usize> IndexMut<usize> for MaybeUninit<[T; N]> {
1175 | |     fn index_mut(&mut self, index: usize) -> &mut Self::Output {
1176 | |         IndexMut::index_mut(self.as_mut(), index)
1177 | |     }
1178 | | }
1178 | | }
     | |_^

error: implementation has missing stability attribute
    --> library/core/src/mem/maybe_uninit.rs:1180:1
     |
1180 | / impl<T, const N: usize> AsRef<[MaybeUninit<T>]> for MaybeUninit<[T; N]> {
1181 | |     #[inline]
1182 | |     fn as_ref(&self) -> &[MaybeUninit<T>] {
1183 | |         let data = self.as_ptr().cast::<MaybeUninit<T>>();
1187 | |     }
1188 | | }
     | |_^


error: implementation has missing stability attribute
    --> library/core/src/mem/maybe_uninit.rs:1190:1
     |
1190 | / impl<T, const N: usize> AsMut<[MaybeUninit<T>]> for MaybeUninit<[T; N]> {
1191 | |     #[inline]
1192 | |     fn as_mut(&mut self) -> &mut [MaybeUninit<T>] {
1193 | |         let data = self.as_mut_ptr().cast::<MaybeUninit<T>>();
1197 | |     }
1198 | | }
     | |_^


error: implementation has missing stability attribute
    --> library/core/src/mem/maybe_uninit.rs:1200:1
     |
1200 | / impl<'a, T, const N: usize> IntoIterator for &'a mut MaybeUninit<[T; N]> {
1201 | |     type Item = &'a mut MaybeUninit<T>;
1202 | |     type IntoIter = IterMut<'a, MaybeUninit<T>>;
...    |
1206 | |     }
1207 | | }
     | |_^
     | |_^

error: missing documentation for an associated function
    --> library/core/src/mem/maybe_uninit.rs:1211:5
     |
1211 |     pub fn iter_mut(&mut self) -> IterMut<'_, MaybeUninit<T>> {
     |
     |
     = note: `-D missing-docs` implied by `-D warnings`
error: could not compile `core` due to 6 previous errors
Build completed unsuccessfully in 0:01:16
