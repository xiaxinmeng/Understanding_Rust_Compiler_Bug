rust
error[E0119]: conflicting implementations of trait `core::convert::AsRef<_>` for type `boxed::Box<_>`:
   --> src/liballoc/boxed.rs:855:1
    |
855 | / impl<T: ?Sized> AsRef<T> for Box<T> {
856 | |     fn as_ref(&self) -> &T {
857 | |         &**self
858 | |     }
859 | | }
    | |_^
    |
    = note: conflicting implementation in crate `core`

error[E0119]: conflicting implementations of trait `core::convert::AsRef<_>` for type `arc::Arc<_>`:
    --> src/liballoc/arc.rs:1857:1
     |
1857 | / impl<T: ?Sized> AsRef<T> for Arc<T> {
1858 | |     fn as_ref(&self) -> &T {
1859 | |         &**self
1860 | |     }
1861 | | }
     | |_^
     |
     = note: conflicting implementation in crate `core`

error[E0119]: conflicting implementations of trait `core::convert::AsRef<_>` for type `rc::Rc<_>`:
    --> src/liballoc/rc.rs:1798:1
     |
1798 | / impl<T: ?Sized> AsRef<T> for Rc<T> {
1799 | |     fn as_ref(&self) -> &T {
1800 | |         &**self
1801 | |     }
1802 | | }
     | |_^
     |
     = note: conflicting implementation in crate `core`

error[E0119]: conflicting implementations of trait `core::convert::AsRef<_>` for type `borrow::Cow<'_, _>`:
   --> src/liballoc/borrow.rs:359:1
    |
359 | / impl<'a, T: ?Sized + ToOwned> AsRef<T> for Cow<'a, T> {
360 | |     fn as_ref(&self) -> &T {
361 | |         self
362 | |     }
363 | | }
    | |_^
    |
    = note: conflicting implementation in crate `core`

error[E0119]: conflicting implementations of trait `core::convert::AsRef<vec::Vec<_>>` for type `vec::Vec<_>`:
    --> src/liballoc/vec.rs:2150:1
     |
2150 | / impl<T> AsRef<Vec<T>> for Vec<T> {
2151 | |     fn as_ref(&self) -> &Vec<T> {
2152 | |         self
2153 | |     }
2154 | | }
     | |_^
     |
     = note: conflicting implementation in crate `core`

error[E0119]: conflicting implementations of trait `core::convert::AsMut<_>` for type `boxed::Box<_>`:
   --> src/liballoc/boxed.rs:862:1
    |
862 | / impl<T: ?Sized> AsMut<T> for Box<T> {
863 | |     fn as_mut(&mut self) -> &mut T {
864 | |         &mut **self
865 | |     }
866 | | }
    | |_^
    |
    = note: conflicting implementation in crate `core`

error[E0119]: conflicting implementations of trait `core::convert::AsMut<vec::Vec<_>>` for type `vec::Vec<_>`:
    --> src/liballoc/vec.rs:2157:1
     |
2157 | / impl<T> AsMut<Vec<T>> for Vec<T> {
2158 | |     fn as_mut(&mut self) -> &mut Vec<T> {
2159 | |         self
2160 | |     }
2161 | | }
     | |_^
     |
     = note: conflicting implementation in crate `core`

