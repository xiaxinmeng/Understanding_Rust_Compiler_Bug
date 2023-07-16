plain
   Compiling tracing-subscriber v0.3.3
   Compiling thorin-dwp v0.1.1
   Compiling rustc_index v0.0.0 (/checkout/compiler/rustc_index)
   Compiling rustc_data_structures v0.0.0 (/checkout/compiler/rustc_data_structures)
error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
   --> compiler/rustc_data_structures/src/sso/map.rs:140:27
    |
140 |     pub fn keys(&self) -> impl Iterator<Item = &'_ K> {
    |
    |
note: hidden type `EitherIter<Map<std::slice::Iter<'<empty>, (K, V)>, [closure@compiler/rustc_data_structures/src/sso/map.rs:142:75: 142:86]>, std::collections::hash_map::Keys<'<empty>, K, V>>` captures lifetime smaller than the function body
   --> compiler/rustc_data_structures/src/sso/map.rs:140:27
    |
140 |     pub fn keys(&self) -> impl Iterator<Item = &'_ K> {


error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
   --> compiler/rustc_data_structures/src/sso/map.rs:149:29
    |
149 |     pub fn values(&self) -> impl Iterator<Item = &'_ V> {
    |
    |
note: hidden type `EitherIter<Map<std::slice::Iter<'<empty>, (K, V)>, [closure@compiler/rustc_data_structures/src/sso/map.rs:151:75: 151:86]>, std::collections::hash_map::Values<'<empty>, K, V>>` captures lifetime smaller than the function body
   --> compiler/rustc_data_structures/src/sso/map.rs:149:29
    |
149 |     pub fn values(&self) -> impl Iterator<Item = &'_ V> {


error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
   --> compiler/rustc_data_structures/src/sso/map.rs:158:37
    |
158 |     pub fn values_mut(&mut self) -> impl Iterator<Item = &'_ mut V> {
    |
    |
note: hidden type `EitherIter<Map<std::slice::IterMut<'<empty>, (K, V)>, [closure@compiler/rustc_data_structures/src/sso/map.rs:160:79: 160:90]>, std::collections::hash_map::ValuesMut<'<empty>, K, V>>` captures lifetime smaller than the function body
   --> compiler/rustc_data_structures/src/sso/map.rs:158:37
    |
158 |     pub fn values_mut(&mut self) -> impl Iterator<Item = &'_ mut V> {


error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
   --> compiler/rustc_data_structures/src/sso/map.rs:167:32
    |
167 |     pub fn drain(&mut self) -> impl Iterator<Item = (K, V)> + '_ {
    |
    |
note: hidden type `EitherIter<arrayvec::Drain<'<empty>, (K, V), 8_usize>, std::collections::hash_map::Drain<'<empty>, K, V>>` captures lifetime smaller than the function body
   --> compiler/rustc_data_structures/src/sso/map.rs:167:32
    |
167 |     pub fn drain(&mut self) -> impl Iterator<Item = (K, V)> + '_ {

   Compiling tracing-tree v0.2.0
   Compiling chalk-solve v0.75.0
   Compiling rustc_log v0.0.0 (/checkout/compiler/rustc_log)
