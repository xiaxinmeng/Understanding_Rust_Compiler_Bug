plain
2019-12-19T05:41:01.4106296Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-19T05:41:01.4335186Z ##[command]git config gc.auto 0
2019-12-19T05:41:01.4412772Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-19T05:41:01.4468277Z ##[command]git config --get-all http.proxy
2019-12-19T05:41:01.4614289Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67415/merge:refs/remotes/pull/67415/merge
---
2019-12-19T05:46:38.8601067Z Successfully built 2673061d01f0
2019-12-19T05:46:38.9889455Z Successfully tagged rust-ci:latest
2019-12-19T05:46:39.0402304Z Built container sha256:2673061d01f0c246c4a386ff392cde3ea35463dcb23994c380a6c60e1c7d9514
2019-12-19T05:46:39.0429263Z Uploading finished image to https://rust-lang-ci-sccache2.s3.amazonaws.com/docker/a4e8b7e8bb02ac2100a242c404ec9cad308c2d13ab7005efa7a2f5ca462ea44fe3b309afeb06c439a0091371f6a14487ea26f96e8916366f1b586cbeecbe4c35
2019-12-19T05:47:40.6389396Z upload failed: - to s3://rust-lang-ci-sccache2/docker/a4e8b7e8bb02ac2100a242c404ec9cad308c2d13ab7005efa7a2f5ca462ea44fe3b309afeb06c439a0091371f6a14487ea26f96e8916366f1b586cbeecbe4c35 An error occurred (InvalidAccessKeyId) when calling the CreateMultipartUpload operation: The AWS Access Key Id you provided does not exist in our records.
2019-12-19T05:47:41.7731642Z [CI_JOB_NAME=mingw-check]
2019-12-19T05:47:41.7766323Z == clock drift check ==
2019-12-19T05:47:41.7809427Z   local time: Thu Dec 19 05:47:41 UTC 2019
2019-12-19T05:47:41.9250935Z   network time: Thu, 19 Dec 2019 05:47:41 GMT
---
2019-12-19T05:49:33.7254129Z     Checking panic_abort v0.0.0 (/checkout/src/libpanic_abort)
2019-12-19T05:49:33.7730646Z     Checking rustc-std-workspace-alloc v1.99.0 (/checkout/src/tools/rustc-std-workspace-alloc)
2019-12-19T05:49:33.9648721Z     Checking backtrace v0.3.40
2019-12-19T05:49:34.1810075Z     Checking panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
2019-12-19T05:49:36.6745333Z error[E0520]: `write` specializes an item from a parent `impl`, but that item is not marked `default`
2019-12-19T05:49:36.6747023Z    --> src/libstd/io/cursor.rs:337:5
2019-12-19T05:49:36.6747509Z     |
2019-12-19T05:49:36.6748382Z 337 | /     fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
2019-12-19T05:49:36.6748968Z 338 | |         slice_write(&mut self.pos, self.inner, buf)
2019-12-19T05:49:36.6749583Z 339 | |     }
2019-12-19T05:49:36.6750044Z     | |_____^ cannot specialize default item `write`
2019-12-19T05:49:36.6750390Z ...
2019-12-19T05:49:36.6750845Z 396 | / impl<T: AsMut<[u8]>> Write for Cursor<T> {
2019-12-19T05:49:36.6751274Z 397 | |     #[inline]
2019-12-19T05:49:36.6751743Z 398 | |     fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
2019-12-19T05:49:36.6752563Z 399 | |         self.inner.as_mut().write(buf)
2019-12-19T05:49:36.6753306Z ...   |
2019-12-19T05:49:36.6753799Z 408 | |     fn flush(&mut self) -> io::Result<()> { Ok(()) }
2019-12-19T05:49:36.6754675Z     | |_- parent `impl` is here
2019-12-19T05:49:36.6755067Z     |
2019-12-19T05:49:36.6755067Z     |
2019-12-19T05:49:36.6755928Z     = note: to specialize, `write` in the parent `impl` must be marked `default`
2019-12-19T05:49:36.6756199Z 
2019-12-19T05:49:36.6763734Z error[E0520]: `write_vectored` specializes an item from a parent `impl`, but that item is not marked `default`
2019-12-19T05:49:36.6764063Z    --> src/libstd/io/cursor.rs:342:5
2019-12-19T05:49:36.6764273Z     |
2019-12-19T05:49:36.6764596Z 342 | /     fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
2019-12-19T05:49:36.6764928Z 343 | |         slice_write_vectored(&mut self.pos, self.inner, bufs)
2019-12-19T05:49:36.6765190Z 344 | |     }
2019-12-19T05:49:36.6766011Z     | |_____^ cannot specialize default item `write_vectored`
2019-12-19T05:49:36.6766286Z ...
2019-12-19T05:49:36.6766786Z 396 | / impl<T: AsMut<[u8]>> Write for Cursor<T> {
2019-12-19T05:49:36.6767170Z 397 | |     #[inline]
2019-12-19T05:49:36.6767514Z 398 | |     fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
2019-12-19T05:49:36.6767836Z 399 | |         self.inner.as_mut().write(buf)
2019-12-19T05:49:36.6768371Z ...   |
2019-12-19T05:49:36.6768715Z 408 | |     fn flush(&mut self) -> io::Result<()> { Ok(()) }
2019-12-19T05:49:36.6769863Z     | |_- parent `impl` is here
2019-12-19T05:49:36.6770552Z     |
2019-12-19T05:49:36.6770552Z     |
2019-12-19T05:49:36.6770909Z     = note: to specialize, `write_vectored` in the parent `impl` must be marked `default`
2019-12-19T05:49:36.6770967Z 
2019-12-19T05:49:36.6790165Z error[E0520]: `flush` specializes an item from a parent `impl`, but that item is not marked `default`
2019-12-19T05:49:36.6790651Z    --> src/libstd/io/cursor.rs:347:5
2019-12-19T05:49:36.6791051Z     |
2019-12-19T05:49:36.6791470Z 347 |       fn flush(&mut self) -> io::Result<()> { Ok(()) }
2019-12-19T05:49:36.6792130Z     |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot specialize default item `flush`
2019-12-19T05:49:36.6792496Z ...
2019-12-19T05:49:36.6792909Z 396 | / impl<T: AsMut<[u8]>> Write for Cursor<T> {
2019-12-19T05:49:36.6793337Z 397 | |     #[inline]
2019-12-19T05:49:36.6793788Z 398 | |     fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
2019-12-19T05:49:36.6794226Z 399 | |         self.inner.as_mut().write(buf)
2019-12-19T05:49:36.6794592Z ...   |
2019-12-19T05:49:36.6795030Z 408 | |     fn flush(&mut self) -> io::Result<()> { Ok(()) }
2019-12-19T05:49:36.6796561Z     | |_- parent `impl` is here
2019-12-19T05:49:36.6796991Z     |
2019-12-19T05:49:36.6796991Z     |
2019-12-19T05:49:36.6797487Z     = note: to specialize, `flush` in the parent `impl` must be marked `default`
2019-12-19T05:49:36.6797697Z 
2019-12-19T05:49:36.6803036Z error[E0520]: `write` specializes an item from a parent `impl`, but that item is not marked `default`
2019-12-19T05:49:36.6808839Z    --> src/libstd/io/cursor.rs:352:5
2019-12-19T05:49:36.6813036Z     |
2019-12-19T05:49:36.6814099Z 352 | /     fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
2019-12-19T05:49:36.6814671Z 353 | |         vec_write(&mut self.pos, self.inner, buf)
2019-12-19T05:49:36.6816120Z 354 | |     }
2019-12-19T05:49:36.6816527Z     | |_____^ cannot specialize default item `write`
2019-12-19T05:49:36.6816759Z ...
2019-12-19T05:49:36.6817647Z 396 | / impl<T: AsMut<[u8]>> Write for Cursor<T> {
2019-12-19T05:49:36.6817963Z 397 | |     #[inline]
2019-12-19T05:49:36.6818538Z 398 | |     fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
2019-12-19T05:49:36.6818870Z 399 | |         self.inner.as_mut().write(buf)
2019-12-19T05:49:36.6819431Z ...   |
2019-12-19T05:49:36.6820150Z 408 | |     fn flush(&mut self) -> io::Result<()> { Ok(()) }
2019-12-19T05:49:36.6820688Z     | |_- parent `impl` is here
2019-12-19T05:49:36.6820859Z     |
2019-12-19T05:49:36.6820859Z     |
2019-12-19T05:49:36.6821740Z     = note: to specialize, `write` in the parent `impl` must be marked `default`
2019-12-19T05:49:36.6821806Z 
2019-12-19T05:49:36.6822074Z error[E0520]: `write_vectored` specializes an item from a parent `impl`, but that item is not marked `default`
2019-12-19T05:49:36.6822275Z    --> src/libstd/io/cursor.rs:356:5
2019-12-19T05:49:36.6822455Z     |
2019-12-19T05:49:36.6822724Z 356 | /     fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
2019-12-19T05:49:36.6823013Z 357 | |         vec_write_vectored(&mut self.pos, self.inner, bufs)
2019-12-19T05:49:36.6823273Z 358 | |     }
2019-12-19T05:49:36.6823532Z     | |_____^ cannot specialize default item `write_vectored`
2019-12-19T05:49:36.6823694Z ...
2019-12-19T05:49:36.6824150Z 396 | / impl<T: AsMut<[u8]>> Write for Cursor<T> {
2019-12-19T05:49:36.6824568Z 397 | |     #[inline]
2019-12-19T05:49:36.6824862Z 398 | |     fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
2019-12-19T05:49:36.6825130Z 399 | |         self.inner.as_mut().write(buf)
2019-12-19T05:49:36.6825331Z ...   |
2019-12-19T05:49:36.6826019Z 408 | |     fn flush(&mut self) -> io::Result<()> { Ok(()) }
2019-12-19T05:49:36.6826817Z     | |_- parent `impl` is here
2019-12-19T05:49:36.6827054Z     |
2019-12-19T05:49:36.6827054Z     |
2019-12-19T05:49:36.6827362Z     = note: to specialize, `write_vectored` in the parent `impl` must be marked `default`
2019-12-19T05:49:36.6827401Z 
2019-12-19T05:49:36.6827808Z error[E0520]: `flush` specializes an item from a parent `impl`, but that item is not marked `default`
2019-12-19T05:49:36.6828064Z    --> src/libstd/io/cursor.rs:361:5
2019-12-19T05:49:36.6828404Z     |
2019-12-19T05:49:36.6828726Z 361 |       fn flush(&mut self) -> io::Result<()> { Ok(()) }
2019-12-19T05:49:36.6829265Z     |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot specialize default item `flush`
2019-12-19T05:49:36.6829455Z ...
2019-12-19T05:49:36.6829708Z 396 | / impl<T: AsMut<[u8]>> Write for Cursor<T> {
2019-12-19T05:49:36.6829942Z 397 | |     #[inline]
2019-12-19T05:49:36.6830225Z 398 | |     fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
2019-12-19T05:49:36.6830477Z 399 | |         self.inner.as_mut().write(buf)
2019-12-19T05:49:36.6830694Z ...   |
2019-12-19T05:49:36.6830953Z 408 | |     fn flush(&mut self) -> io::Result<()> { Ok(()) }
2019-12-19T05:49:36.6831419Z     | |_- parent `impl` is here
2019-12-19T05:49:36.6831586Z     |
2019-12-19T05:49:36.6831586Z     |
2019-12-19T05:49:36.6831822Z     = note: to specialize, `flush` in the parent `impl` must be marked `default`
2019-12-19T05:49:36.6831868Z 
2019-12-19T05:49:36.6832110Z error[E0520]: `write` specializes an item from a parent `impl`, but that item is not marked `default`
2019-12-19T05:49:36.6832319Z    --> src/libstd/io/cursor.rs:366:5
2019-12-19T05:49:36.6832500Z     |
2019-12-19T05:49:36.6832762Z 366 | /     fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
2019-12-19T05:49:36.6833024Z 367 | |         vec_write(&mut self.pos, &mut self.inner, buf)
2019-12-19T05:49:36.6833266Z 368 | |     }
2019-12-19T05:49:36.6833510Z     | |_____^ cannot specialize default item `write`
2019-12-19T05:49:36.6833666Z ...
2019-12-19T05:49:36.6833930Z 396 | / impl<T: AsMut<[u8]>> Write for Cursor<T> {
2019-12-19T05:49:36.6834168Z 397 | |     #[inline]
2019-12-19T05:49:36.6834544Z 398 | |     fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
2019-12-19T05:49:36.6834837Z 399 | |         self.inner.as_mut().write(buf)
2019-12-19T05:49:36.6835028Z ...   |
2019-12-19T05:49:36.6835300Z 408 | |     fn flush(&mut self) -> io::Result<()> { Ok(()) }
2019-12-19T05:49:36.6836227Z     | |_- parent `impl` is here
2019-12-19T05:49:36.6836967Z     |
2019-12-19T05:49:36.6836967Z     |
2019-12-19T05:49:36.6837280Z     = note: to specialize, `write` in the parent `impl` must be marked `default`
2019-12-19T05:49:36.6837318Z 
2019-12-19T05:49:36.6873747Z error[E0520]: `write_vectored` specializes an item from a parent `impl`, but that item is not marked `default`
2019-12-19T05:49:36.6874077Z    --> src/libstd/io/cursor.rs:370:5
2019-12-19T05:49:36.6874274Z     |
2019-12-19T05:49:36.6874656Z 370 | /     fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
2019-12-19T05:49:36.6874960Z 371 | |         vec_write_vectored(&mut self.pos, &mut self.inner, bufs)
2019-12-19T05:49:36.6875248Z 372 | |     }
2019-12-19T05:49:36.6875923Z     | |_____^ cannot specialize default item `write_vectored`
2019-12-19T05:49:36.6876182Z ...
2019-12-19T05:49:36.6876520Z 396 | / impl<T: AsMut<[u8]>> Write for Cursor<T> {
2019-12-19T05:49:36.6876822Z 397 | |     #[inline]
2019-12-19T05:49:36.6877160Z 398 | |     fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
2019-12-19T05:49:36.6877501Z 399 | |         self.inner.as_mut().write(buf)
2019-12-19T05:49:36.6877753Z ...   |
2019-12-19T05:49:36.6878086Z 408 | |     fn flush(&mut self) -> io::Result<()> { Ok(()) }
2019-12-19T05:49:36.6878687Z     | |_- parent `impl` is here
2019-12-19T05:49:36.6878920Z     |
2019-12-19T05:49:36.6878920Z     |
2019-12-19T05:49:36.6879393Z     = note: to specialize, `write_vectored` in the parent `impl` must be marked `default`
2019-12-19T05:49:36.6879428Z 
2019-12-19T05:49:36.6879676Z error[E0520]: `flush` specializes an item from a parent `impl`, but that item is not marked `default`
2019-12-19T05:49:36.6879915Z    --> src/libstd/io/cursor.rs:375:5
2019-12-19T05:49:36.6880087Z     |
2019-12-19T05:49:36.6880471Z 375 |       fn flush(&mut self) -> io::Result<()> { Ok(()) }
2019-12-19T05:49:36.6880849Z     |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot specialize default item `flush`
2019-12-19T05:49:36.6881198Z ...
2019-12-19T05:49:36.6881479Z 396 | / impl<T: AsMut<[u8]>> Write for Cursor<T> {
2019-12-19T05:49:36.6881728Z 397 | |     #[inline]
2019-12-19T05:49:36.6882188Z 398 | |     fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
2019-12-19T05:49:36.6882614Z 399 | |         self.inner.as_mut().write(buf)
2019-12-19T05:49:36.6882834Z ...   |
2019-12-19T05:49:36.6883118Z 408 | |     fn flush(&mut self) -> io::Result<()> { Ok(()) }
2019-12-19T05:49:36.6883636Z     | |_- parent `impl` is here
2019-12-19T05:49:36.6883821Z     |
2019-12-19T05:49:36.6883821Z     |
2019-12-19T05:49:36.6884103Z     = note: to specialize, `flush` in the parent `impl` must be marked `default`
2019-12-19T05:49:36.6884183Z 
2019-12-19T05:49:36.6907540Z error[E0520]: `write` specializes an item from a parent `impl`, but that item is not marked `default`
2019-12-19T05:49:36.6908430Z    --> src/libstd/io/cursor.rs:381:5
2019-12-19T05:49:36.6909124Z     |
2019-12-19T05:49:36.6909819Z 381 | /     fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
2019-12-19T05:49:36.6910303Z 382 | |         slice_write(&mut self.pos, &mut self.inner, buf)
2019-12-19T05:49:36.6910707Z 383 | |     }
2019-12-19T05:49:36.6911152Z     | |_____^ cannot specialize default item `write`
2019-12-19T05:49:36.6911661Z ...
2019-12-19T05:49:36.6912105Z 396 | / impl<T: AsMut<[u8]>> Write for Cursor<T> {
2019-12-19T05:49:36.6912540Z 397 | |     #[inline]
2019-12-19T05:49:36.6912982Z 398 | |     fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
2019-12-19T05:49:36.6913431Z 399 | |         self.inner.as_mut().write(buf)
2019-12-19T05:49:36.6913790Z ...   |
2019-12-19T05:49:36.6914212Z 408 | |     fn flush(&mut self) -> io::Result<()> { Ok(()) }
2019-12-19T05:49:36.6915184Z     | |_- parent `impl` is here
2019-12-19T05:49:36.6915964Z     |
2019-12-19T05:49:36.6915964Z     |
2019-12-19T05:49:36.6916671Z     = note: to specialize, `write` in the parent `impl` must be marked `default`
2019-12-19T05:49:36.6916909Z 
2019-12-19T05:49:36.6917531Z error[E0520]: `write_vectored` specializes an item from a parent `impl`, but that item is not marked `default`
2019-12-19T05:49:36.6918055Z    --> src/libstd/io/cursor.rs:386:5
2019-12-19T05:49:36.6918515Z     |
2019-12-19T05:49:36.6919253Z 386 | /     fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
2019-12-19T05:49:36.6919863Z 387 | |         slice_write_vectored(&mut self.pos, &mut self.inner, bufs)
2019-12-19T05:49:36.6920305Z 388 | |     }
2019-12-19T05:49:36.6920725Z     | |_____^ cannot specialize default item `write_vectored`
2019-12-19T05:49:36.6921064Z ...
2019-12-19T05:49:36.6921472Z 396 | / impl<T: AsMut<[u8]>> Write for Cursor<T> {
2019-12-19T05:49:36.6921878Z 397 | |     #[inline]
2019-12-19T05:49:36.6922300Z 398 | |     fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
2019-12-19T05:49:36.6922723Z 399 | |         self.inner.as_mut().write(buf)
2019-12-19T05:49:36.6923093Z ...   |
2019-12-19T05:49:36.6923514Z 408 | |     fn flush(&mut self) -> io::Result<()> { Ok(()) }
2019-12-19T05:49:36.6924328Z     | |_- parent `impl` is here
2019-12-19T05:49:36.6924650Z     |
2019-12-19T05:49:36.6924650Z     |
2019-12-19T05:49:36.6925072Z     = note: to specialize, `write_vectored` in the parent `impl` must be marked `default`
2019-12-19T05:49:36.6925223Z 
2019-12-19T05:49:36.6925610Z error[E0520]: `flush` specializes an item from a parent `impl`, but that item is not marked `default`
2019-12-19T05:49:36.6926520Z    --> src/libstd/io/cursor.rs:391:5
2019-12-19T05:49:36.6926935Z     |
2019-12-19T05:49:36.6927455Z 391 |       fn flush(&mut self) -> io::Result<()> { Ok(()) }
2019-12-19T05:49:36.6928262Z     |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot specialize default item `flush`
2019-12-19T05:49:36.6928748Z ...
2019-12-19T05:49:36.6929427Z 396 | / impl<T: AsMut<[u8]>> Write for Cursor<T> {
2019-12-19T05:49:36.6929995Z 397 | |     #[inline]
2019-12-19T05:49:36.6930446Z 398 | |     fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
2019-12-19T05:49:36.6930959Z 399 | |         self.inner.as_mut().write(buf)
2019-12-19T05:49:36.6931400Z ...   |
2019-12-19T05:49:36.6931815Z 408 | |     fn flush(&mut self) -> io::Result<()> { Ok(()) }
2019-12-19T05:49:36.6932952Z     | |_- parent `impl` is here
2019-12-19T05:49:36.6933120Z     |
2019-12-19T05:49:36.6933120Z     |
2019-12-19T05:49:36.6933378Z     = note: to specialize, `flush` in the parent `impl` must be marked `default`
2019-12-19T05:49:38.0510388Z error: aborting due to 12 previous errors
2019-12-19T05:49:38.0511028Z 
2019-12-19T05:49:38.0511573Z For more information about this error, try `rustc --explain E0520`.
2019-12-19T05:49:38.0641661Z error: could not compile `std`.
---
2019-12-19T05:49:38.0772107Z   local time: Thu Dec 19 05:49:38 UTC 2019
2019-12-19T05:49:38.3520683Z   network time: Thu, 19 Dec 2019 05:49:38 GMT
2019-12-19T05:49:38.3525152Z == end clock drift check ==
2019-12-19T05:49:46.1078948Z 
2019-12-19T05:49:46.1191887Z ##[error]Bash exited with code '1'.
2019-12-19T05:49:46.1223432Z ##[section]Starting: Checkout
2019-12-19T05:49:46.1225487Z ==============================================================================
2019-12-19T05:49:46.1225548Z Task         : Get sources
2019-12-19T05:49:46.1225597Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
