
error[E0119]: conflicting implementations of trait `convert::AsRef<&_>` for type `&_`:
   --> /checkout/src/libcore/convert.rs:237:1
    |
229 |   impl<'a, T: ?Sized> AsRef<T> for T {
    |  _- starting here...
230 | |     fn as_ref(&self) -> &T {
231 | |         self
232 | |     }
233 | | }
    | |_- ...ending here: first implementation here
...
237 |   impl<'a, T: ?Sized, U: ?Sized> AsRef<U> for &'a T where T: AsRef<U> {
    |  _^ starting here...
238 | |     fn as_ref(&self) -> &U {
239 | |         <T as AsRef<U>>::as_ref(*self)
240 | |     }
241 | | }
    | |_^ ...ending here: conflicting implementation for `&_`

error[E0119]: conflicting implementations of trait `convert::AsRef<&mut _>` for type `&mut _`:
   --> /checkout/src/libcore/convert.rs:245:1
    |
229 |   impl<'a, T: ?Sized> AsRef<T> for T {
    |  _- starting here...
230 | |     fn as_ref(&self) -> &T {
231 | |         self
232 | |     }
233 | | }
    | |_- ...ending here: first implementation here
...
245 |   impl<'a, T: ?Sized, U: ?Sized> AsRef<U> for &'a mut T where T: AsRef<U> {
    |  _^ starting here...
246 | |     fn as_ref(&self) -> &U {
247 | |         <T as AsRef<U>>::as_ref(*self)
248 | |     }
249 | | }
    | |_^ ...ending here: conflicting implementation for `&mut _`

error[E0119]: conflicting implementations of trait `convert::AsMut<&mut _>` for type `&mut _`:
   --> /checkout/src/libcore/convert.rs:269:1
    |
261 |   impl<'a, T: ?Sized> AsMut<T> for T {
    |  _- starting here...
262 | |     fn as_mut(&mut self) -> &mut T {
263 | |         self
264 | |     }
265 | | }
    | |_- ...ending here: first implementation here
...
269 |   impl<'a, T: ?Sized, U: ?Sized> AsMut<U> for &'a mut T where T: AsMut<U> {
    |  _^ starting here...
270 | |     fn as_mut(&mut self) -> &mut U {
271 | |         (*self).as_mut()
272 | |     }
273 | | }
    | |_^ ...ending here: conflicting implementation for `&mut _`

error: aborting due to 3 previous errors
