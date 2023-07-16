
error: reached the type-length limit while instantiating `<futures::future::Ready<std::res...y>::new_service::{{closure}}#0]>`
   --> /home/xxx/.cargo/registry/src/github.com-1ecc6299db9ec823/futures-util-0.3.5/src/future/future/mod.rs:142:5
    |
142 | /     fn map<U, F>(self, f: F) -> Map<Self, F>
143 | |     where
144 | |         F: FnOnce(Self::Output) -> U,
145 | |         Self: Sized,
146 | |     {
147 | |         assert_future::<U, _>(Map::new(self, f))
148 | |     }
    | |_____^
    |
    = note: consider adding a `#![type_length_limit="1471465"]` attribute to your crate

error: aborting due to previous error
