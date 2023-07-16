plain
    Checking rand v0.7.3
    Checking core v0.0.0 (/checkout/library/core)
    Checking std v0.0.0 (/checkout/library/std)
    Checking alloc v0.0.0 (/checkout/library/alloc)
error[E0107]: this associated function takes 1 generic argument but 2 generic arguments were supplied
    |
    |
145 |             .provide_value::<String, _>(|| "bye".to_owned());
    |              ^^^^^^^^^^^^^           - help: remove this generic argument
    |              expected 1 generic argument
    |
    |
note: associated function defined here, with 1 generic parameter: `T`
    |
    |
901 |     pub fn provide_value<T>(&mut self, fulfil: impl FnOnce() -> T) -> &mut Self
    |            ^^^^^^^^^^^^^ -
    = note: `impl Trait` cannot be explicitly specified as a generic argument

error[E0107]: this function takes 1 generic argument but 2 generic arguments were supplied
    |
    |
185 |         request_ref::<T, _>(self)
    |         ^^^^^^^^^^^      - help: remove this generic argument
    |         expected 1 generic argument
    |
    |
note: function defined here, with 1 generic parameter: `T`
    |
    |
847 | pub fn request_ref<'a, T>(provider: &'a (impl Provider + ?Sized)) -> Option<&'a T>
    |        ^^^^^^^^^^^     -
    = note: `impl Trait` cannot be explicitly specified as a generic argument

error[E0107]: this function takes 1 generic argument but 2 generic arguments were supplied
    |
    |
154 |     assert_eq!(&**request_ref::<String, _>(obj).unwrap(), "hello");
    |                   ^^^^^^^^^^^           - help: remove this generic argument
    |                   expected 1 generic argument
    |
    |
note: function defined here, with 1 generic parameter: `T`
    |
    |
847 | pub fn request_ref<'a, T>(provider: &'a (impl Provider + ?Sized)) -> Option<&'a T>
    |        ^^^^^^^^^^^     -
    = note: `impl Trait` cannot be explicitly specified as a generic argument

error[E0107]: this function takes 1 generic argument but 2 generic arguments were supplied
    |
    |
155 |     assert_eq!(&*request_value::<String, _>(obj).unwrap(), "bye");
    |                  ^^^^^^^^^^^^^           - help: remove this generic argument
    |                  expected 1 generic argument
    |
    |
note: function defined here, with 1 generic parameter: `T`
    |
    |
825 | pub fn request_value<'a, T>(provider: &'a (impl Provider + ?Sized)) -> Option<T>
    |        ^^^^^^^^^^^^^     -
    = note: `impl Trait` cannot be explicitly specified as a generic argument

error[E0107]: this function takes 1 generic argument but 2 generic arguments were supplied
    |
    |
156 |     assert_eq!(request_value::<u8, _>(obj), None);
    |                ^^^^^^^^^^^^^       - help: remove this generic argument
    |                expected 1 generic argument
    |
    |
note: function defined here, with 1 generic parameter: `T`
    |
    |
825 | pub fn request_value<'a, T>(provider: &'a (impl Provider + ?Sized)) -> Option<T>
    |        ^^^^^^^^^^^^^     -
    = note: `impl Trait` cannot be explicitly specified as a generic argument

error[E0107]: this function takes 1 generic argument but 2 generic arguments were supplied
    |
    |
164 |     assert_eq!(&**request_ref::<String, _>(&*obj).unwrap(), "hello");
    |                   ^^^^^^^^^^^           - help: remove this generic argument
    |                   expected 1 generic argument
    |
    |
note: function defined here, with 1 generic parameter: `T`
    |
    |
847 | pub fn request_ref<'a, T>(provider: &'a (impl Provider + ?Sized)) -> Option<&'a T>
    |        ^^^^^^^^^^^     -
    = note: `impl Trait` cannot be explicitly specified as a generic argument

error[E0107]: this function takes 1 generic argument but 2 generic arguments were supplied
    |
    |
165 |     assert_eq!(&*request_value::<String, _>(&*obj).unwrap(), "bye");
    |                  ^^^^^^^^^^^^^           - help: remove this generic argument
    |                  expected 1 generic argument
    |
    |
note: function defined here, with 1 generic parameter: `T`
    |
    |
825 | pub fn request_value<'a, T>(provider: &'a (impl Provider + ?Sized)) -> Option<T>
    |        ^^^^^^^^^^^^^     -
    = note: `impl Trait` cannot be explicitly specified as a generic argument

error[E0107]: this function takes 1 generic argument but 2 generic arguments were supplied
    |
    |
166 |     assert_eq!(request_value::<u8, _>(&*obj), None);
    |                ^^^^^^^^^^^^^       - help: remove this generic argument
    |                expected 1 generic argument
    |
    |
note: function defined here, with 1 generic parameter: `T`
    |
    |
825 | pub fn request_value<'a, T>(provider: &'a (impl Provider + ?Sized)) -> Option<T>
    |        ^^^^^^^^^^^^^     -
    = note: `impl Trait` cannot be explicitly specified as a generic argument

error[E0107]: this function takes 1 generic argument but 2 generic arguments were supplied
    |
    |
174 |     assert_eq!(&**request_ref::<String, _>(&obj).unwrap(), "hello");
    |                   ^^^^^^^^^^^           - help: remove this generic argument
    |                   expected 1 generic argument
    |
    |
note: function defined here, with 1 generic parameter: `T`
    |
    |
847 | pub fn request_ref<'a, T>(provider: &'a (impl Provider + ?Sized)) -> Option<&'a T>
    |        ^^^^^^^^^^^     -
    = note: `impl Trait` cannot be explicitly specified as a generic argument

error[E0107]: this function takes 1 generic argument but 2 generic arguments were supplied
    |
    |
175 |     assert_eq!(&*request_value::<String, _>(&obj).unwrap(), "bye");
    |                  ^^^^^^^^^^^^^           - help: remove this generic argument
    |                  expected 1 generic argument
    |
    |
note: function defined here, with 1 generic parameter: `T`
    |
    |
825 | pub fn request_value<'a, T>(provider: &'a (impl Provider + ?Sized)) -> Option<T>
    |        ^^^^^^^^^^^^^     -
    = note: `impl Trait` cannot be explicitly specified as a generic argument

error[E0107]: this function takes 1 generic argument but 2 generic arguments were supplied
    |
    |
176 |     assert_eq!(request_value::<u8, _>(&obj), None);
    |                ^^^^^^^^^^^^^       - help: remove this generic argument
    |                expected 1 generic argument
    |
    |
note: function defined here, with 1 generic parameter: `T`
    |
    |
825 | pub fn request_value<'a, T>(provider: &'a (impl Provider + ?Sized)) -> Option<T>
    |        ^^^^^^^^^^^^^     -
    = note: `impl Trait` cannot be explicitly specified as a generic argument
For more information about this error, try `rustc --explain E0107`.
error: could not compile `core` due to 11 previous errors
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:01:46
