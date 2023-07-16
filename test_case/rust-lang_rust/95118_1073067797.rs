plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
    Checking miniz_oxide v0.4.0
    Checking object v0.26.2
    Checking std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
    Checking addr2line v0.16.0
error: implementation has missing stability attribute
   --> library/std/src/os/windows/io/handle.rs:113:1
    |
113 | unsafe impl Send for OwnedHandle {}

error: implementation has missing stability attribute
error: implementation has missing stability attribute
   --> library/std/src/os/windows/io/handle.rs:114:1
    |
114 | unsafe impl Send for HandleOrNull {}

error: implementation has missing stability attribute
error: implementation has missing stability attribute
   --> library/std/src/os/windows/io/handle.rs:115:1
    |
115 | unsafe impl Send for HandleOrInvalid {}

error: implementation has missing stability attribute
error: implementation has missing stability attribute
   --> library/std/src/os/windows/io/handle.rs:116:1
    |
116 | unsafe impl Send for BorrowedHandle<'_> {}

error: implementation has missing stability attribute
error: implementation has missing stability attribute
   --> library/std/src/os/windows/io/handle.rs:117:1
    |
117 | unsafe impl Sync for OwnedHandle {}

error: implementation has missing stability attribute
error: implementation has missing stability attribute
   --> library/std/src/os/windows/io/handle.rs:118:1
    |
118 | unsafe impl Sync for HandleOrNull {}

error: implementation has missing stability attribute
error: implementation has missing stability attribute
   --> library/std/src/os/windows/io/handle.rs:119:1
    |
119 | unsafe impl Sync for HandleOrInvalid {}

error: implementation has missing stability attribute
error: implementation has missing stability attribute
   --> library/std/src/os/windows/io/handle.rs:120:1
    |
120 | unsafe impl Sync for BorrowedHandle<'_> {}

error: implementation has missing stability attribute
error: implementation has missing stability attribute
   --> library/std/src/os/windows/io/handle.rs:144:1
    |
144 | / impl TryFrom<HandleOrNull> for OwnedHandle {
145 | |     type Error = ();
147 | |     #[inline]
...   |
151 | |     }
152 | | }
152 | | }
    | |_^

error: associated function has missing stability attribute
   --> library/std/src/os/windows/io/handle.rs:157:5
    |
157 | /     pub fn try_clone(&self) -> crate::io::Result<Self> {
158 | |         self.duplicate(0, false, c::DUPLICATE_SAME_ACCESS)
    | |_____^

error: implementation has missing stability attribute
error: implementation has missing stability attribute
   --> library/std/src/os/windows/io/handle.rs:194:1
    |
194 | / impl TryFrom<HandleOrInvalid> for OwnedHandle {
195 | |     type Error = ();
197 | |     #[inline]
...   |
201 | |     }
202 | | }
202 | | }
    | |_^

error: implementation has missing stability attribute
   --> library/std/src/os/windows/io/handle.rs:204:1
    |
204 | / impl AsRawHandle for BorrowedHandle<'_> {
205 | |     #[inline]
206 | |     fn as_raw_handle(&self) -> RawHandle {
207 | |         self.handle
209 | | }
    | |_^

error: implementation has missing stability attribute
error: implementation has missing stability attribute
   --> library/std/src/os/windows/io/handle.rs:211:1
    |
211 | / impl AsRawHandle for OwnedHandle {
212 | |     #[inline]
213 | |     fn as_raw_handle(&self) -> RawHandle {
214 | |         self.handle
216 | | }
    | |_^

error: implementation has missing stability attribute
error: implementation has missing stability attribute
   --> library/std/src/os/windows/io/handle.rs:218:1
    |
218 | / impl IntoRawHandle for OwnedHandle {
219 | |     #[inline]
220 | |     fn into_raw_handle(self) -> RawHandle {
221 | |         let handle = self.handle;
224 | |     }
225 | | }
    | |_^


error: implementation has missing stability attribute
   --> library/std/src/os/windows/io/handle.rs:227:1
    |
227 | / impl FromRawHandle for OwnedHandle {
228 | |     #[inline]
229 | |     unsafe fn from_raw_handle(handle: RawHandle) -> Self {
230 | |         Self { handle }
232 | | }
    | |_^

error: associated function has missing stability attribute
error: associated function has missing stability attribute
   --> library/std/src/os/windows/io/handle.rs:250:5
    |
250 | /     pub unsafe fn from_raw_handle(handle: RawHandle) -> Self {
251 | |         Self(OwnedHandle::from_raw_handle(handle))
    | |_____^

error: associated function has missing stability attribute
error: associated function has missing stability attribute
   --> library/std/src/os/windows/io/handle.rs:272:5
    |
272 | /     pub unsafe fn from_raw_handle(handle: RawHandle) -> Self {
273 | |         Self(OwnedHandle::from_raw_handle(handle))
    | |_____^

error: implementation has missing stability attribute
error: implementation has missing stability attribute
   --> library/std/src/os/windows/io/handle.rs:277:1
    |
277 | / impl Drop for OwnedHandle {
278 | |     #[inline]
279 | |     fn drop(&mut self) {
...   |
283 | |     }
284 | | }
    | |_^
    | |_^

error: implementation has missing stability attribute
   --> library/std/src/os/windows/io/handle.rs:286:1
    |
286 | / impl fmt::Debug for BorrowedHandle<'_> {
287 | |     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
288 | |         f.debug_struct("BorrowedHandle").field("handle", &self.handle).finish()
290 | | }
    | |_^

error: implementation has missing stability attribute
error: implementation has missing stability attribute
   --> library/std/src/os/windows/io/handle.rs:292:1
    |
292 | / impl fmt::Debug for OwnedHandle {
293 | |     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
294 | |         f.debug_struct("OwnedHandle").field("handle", &self.handle).finish()
296 | | }
    | |_^

error: associated function has missing stability attribute
error: associated function has missing stability attribute
   --> library/std/src/os/windows/io/handle.rs:314:5
    |
314 |     fn as_handle(&self) -> BorrowedHandle<'_>;

error: implementation has missing stability attribute
error: implementation has missing stability attribute
   --> library/std/src/os/windows/io/handle.rs:333:1
    |
333 | / impl AsHandle for BorrowedHandle<'_> {
334 | |     #[inline]
335 | |     fn as_handle(&self) -> BorrowedHandle<'_> {
336 | |         *self
338 | | }
    | |_^

error: implementation has missing stability attribute
error: implementation has missing stability attribute
   --> library/std/src/os/windows/io/handle.rs:340:1
    |
340 | / impl AsHandle for OwnedHandle {
341 | |     #[inline]
342 | |     fn as_handle(&self) -> BorrowedHandle<'_> {
343 | |         // Safety: `OwnedHandle` and `BorrowedHandle` have the same validity
347 | |     }
348 | | }
    | |_^


error: implementation has missing stability attribute
   --> library/std/src/os/windows/io/handle.rs:350:1
    |
350 | / impl AsHandle for fs::File {
351 | |     #[inline]
352 | |     fn as_handle(&self) -> BorrowedHandle<'_> {
353 | |         self.as_inner().as_handle()
355 | | }
    | |_^

error: implementation has missing stability attribute
error: implementation has missing stability attribute
   --> library/std/src/os/windows/io/handle.rs:357:1
    |
357 | / impl From<fs::File> for OwnedHandle {
358 | |     #[inline]
359 | |     fn from(file: fs::File) -> OwnedHandle {
360 | |         file.into_inner().into_inner().into_inner().into()
362 | | }
    | |_^

error: implementation has missing stability attribute
error: implementation has missing stability attribute
   --> library/std/src/os/windows/io/handle.rs:364:1
    |
364 | / impl From<OwnedHandle> for fs::File {
365 | |     #[inline]
366 | |     fn from(owned: OwnedHandle) -> Self {
367 | |         Self::from_inner(FromInner::from_inner(FromInner::from_inner(owned)))
369 | | }
    | |_^

error: implementation has missing stability attribute
error: implementation has missing stability attribute
   --> library/std/src/os/windows/io/handle.rs:371:1
    |
371 | / impl AsHandle for crate::io::Stdin {
372 | |     #[inline]
373 | |     fn as_handle(&self) -> BorrowedHandle<'_> {
374 | |         unsafe { BorrowedHandle::borrow_raw(self.as_raw_handle()) }
376 | | }
    | |_^

error: implementation has missing stability attribute
error: implementation has missing stability attribute
   --> library/std/src/os/windows/io/handle.rs:378:1
    |
378 | / impl<'a> AsHandle for crate::io::StdinLock<'a> {
379 | |     #[inline]
380 | |     fn as_handle(&self) -> BorrowedHandle<'_> {
381 | |         unsafe { BorrowedHandle::borrow_raw(self.as_raw_handle()) }
383 | | }
    | |_^

error: implementation has missing stability attribute
error: implementation has missing stability attribute
   --> library/std/src/os/windows/io/handle.rs:385:1
    |
385 | / impl AsHandle for crate::io::Stdout {
386 | |     #[inline]
387 | |     fn as_handle(&self) -> BorrowedHandle<'_> {
388 | |         unsafe { BorrowedHandle::borrow_raw(self.as_raw_handle()) }
390 | | }
    | |_^

error: implementation has missing stability attribute
error: implementation has missing stability attribute
   --> library/std/src/os/windows/io/handle.rs:392:1
    |
392 | / impl<'a> AsHandle for crate::io::StdoutLock<'a> {
393 | |     #[inline]
394 | |     fn as_handle(&self) -> BorrowedHandle<'_> {
395 | |         unsafe { BorrowedHandle::borrow_raw(self.as_raw_handle()) }
397 | | }
    | |_^

error: implementation has missing stability attribute
error: implementation has missing stability attribute
   --> library/std/src/os/windows/io/handle.rs:399:1
    |
399 | / impl AsHandle for crate::io::Stderr {
400 | |     #[inline]
401 | |     fn as_handle(&self) -> BorrowedHandle<'_> {
402 | |         unsafe { BorrowedHandle::borrow_raw(self.as_raw_handle()) }
404 | | }
    | |_^

error: implementation has missing stability attribute
error: implementation has missing stability attribute
   --> library/std/src/os/windows/io/handle.rs:406:1
    |
406 | / impl<'a> AsHandle for crate::io::StderrLock<'a> {
407 | |     #[inline]
408 | |     fn as_handle(&self) -> BorrowedHandle<'_> {
409 | |         unsafe { BorrowedHandle::borrow_raw(self.as_raw_handle()) }
411 | | }
    | |_^

error: implementation has missing stability attribute
error: implementation has missing stability attribute
   --> library/std/src/os/windows/io/handle.rs:413:1
    |
413 | / impl AsHandle for crate::process::ChildStdin {
414 | |     #[inline]
415 | |     fn as_handle(&self) -> BorrowedHandle<'_> {
416 | |         unsafe { BorrowedHandle::borrow_raw(self.as_raw_handle()) }
418 | | }
    | |_^

error: implementation has missing stability attribute
error: implementation has missing stability attribute
   --> library/std/src/os/windows/io/handle.rs:420:1
    |
420 | / impl From<crate::process::ChildStdin> for OwnedHandle {
421 | |     #[inline]
422 | |     fn from(child_stdin: crate::process::ChildStdin) -> OwnedHandle {
423 | |         unsafe { OwnedHandle::from_raw_handle(child_stdin.into_raw_handle()) }
425 | | }
    | |_^

error: implementation has missing stability attribute
error: implementation has missing stability attribute
   --> library/std/src/os/windows/io/handle.rs:427:1
    |
427 | / impl AsHandle for crate::process::ChildStdout {
428 | |     #[inline]
429 | |     fn as_handle(&self) -> BorrowedHandle<'_> {
430 | |         unsafe { BorrowedHandle::borrow_raw(self.as_raw_handle()) }
432 | | }
    | |_^

error: implementation has missing stability attribute
error: implementation has missing stability attribute
   --> library/std/src/os/windows/io/handle.rs:434:1
    |
434 | / impl From<crate::process::ChildStdout> for OwnedHandle {
435 | |     #[inline]
436 | |     fn from(child_stdout: crate::process::ChildStdout) -> OwnedHandle {
437 | |         unsafe { OwnedHandle::from_raw_handle(child_stdout.into_raw_handle()) }
439 | | }
    | |_^

error: implementation has missing stability attribute
error: implementation has missing stability attribute
   --> library/std/src/os/windows/io/handle.rs:441:1
    |
441 | / impl AsHandle for crate::process::ChildStderr {
442 | |     #[inline]
443 | |     fn as_handle(&self) -> BorrowedHandle<'_> {
444 | |         unsafe { BorrowedHandle::borrow_raw(self.as_raw_handle()) }
446 | | }
    | |_^

error: implementation has missing stability attribute
error: implementation has missing stability attribute
   --> library/std/src/os/windows/io/handle.rs:448:1
    |
448 | / impl From<crate::process::ChildStderr> for OwnedHandle {
449 | |     #[inline]
450 | |     fn from(child_stderr: crate::process::ChildStderr) -> OwnedHandle {
451 | |         unsafe { OwnedHandle::from_raw_handle(child_stderr.into_raw_handle()) }
453 | | }
    | |_^

error: implementation has missing stability attribute
error: implementation has missing stability attribute
   --> library/std/src/os/windows/io/handle.rs:455:1
    |
455 | / impl<T> AsHandle for crate::thread::JoinHandle<T> {
456 | |     #[inline]
457 | |     fn as_handle(&self) -> BorrowedHandle<'_> {
458 | |         unsafe { BorrowedHandle::borrow_raw(self.as_raw_handle()) }
460 | | }
    | |_^

error: implementation has missing stability attribute
error: implementation has missing stability attribute
   --> library/std/src/os/windows/io/handle.rs:462:1
    |
462 | / impl<T> From<crate::thread::JoinHandle<T>> for OwnedHandle {
463 | |     #[inline]
464 | |     fn from(join_handle: crate::thread::JoinHandle<T>) -> OwnedHandle {
465 | |         join_handle.into_inner().into_handle().into_inner()
467 | | }
    | |_^

error: associated function has missing stability attribute
error: associated function has missing stability attribute
   --> library/std/src/os/windows/io/socket.rs:83:5
    |
83  | /     pub fn try_clone(&self) -> io::Result<Self> {
84  | |         let mut info = unsafe { mem::zeroed::<c::WSAPROTOCOL_INFO>() };
85  | |         let result = unsafe {
86  | |             c::WSADuplicateSocketW(self.as_raw_socket(), c::GetCurrentProcessId(), &mut info)
129 | |         }
130 | |     }
    | |_____^


error: implementation has missing stability attribute
   --> library/std/src/os/windows/io/socket.rs:151:1
    |
151 | / impl AsRawSocket for BorrowedSocket<'_> {
152 | |     #[inline]
153 | |     fn as_raw_socket(&self) -> RawSocket {
154 | |         self.socket
156 | | }
    | |_^

error: implementation has missing stability attribute
error: implementation has missing stability attribute
   --> library/std/src/os/windows/io/socket.rs:158:1
    |
158 | / impl AsRawSocket for OwnedSocket {
159 | |     #[inline]
160 | |     fn as_raw_socket(&self) -> RawSocket {
161 | |         self.socket
163 | | }
    | |_^

error: implementation has missing stability attribute
error: implementation has missing stability attribute
   --> library/std/src/os/windows/io/socket.rs:165:1
    |
165 | / impl IntoRawSocket for OwnedSocket {
166 | |     #[inline]
167 | |     fn into_raw_socket(self) -> RawSocket {
168 | |         let socket = self.socket;
171 | |     }
172 | | }
    | |_^


error: implementation has missing stability attribute
   --> library/std/src/os/windows/io/socket.rs:174:1
    |
174 | / impl FromRawSocket for OwnedSocket {
175 | |     #[inline]
176 | |     unsafe fn from_raw_socket(socket: RawSocket) -> Self {
177 | |         debug_assert_ne!(socket, c::INVALID_SOCKET as RawSocket);
178 | |         Self { socket }
180 | | }
    | |_^

error: implementation has missing stability attribute
error: implementation has missing stability attribute
   --> library/std/src/os/windows/io/socket.rs:182:1
    |
182 | / impl Drop for OwnedSocket {
183 | |     #[inline]
184 | |     fn drop(&mut self) {
...   |
188 | |     }
189 | | }
    | |_^
    | |_^

error: implementation has missing stability attribute
   --> library/std/src/os/windows/io/socket.rs:191:1
    |
191 | / impl fmt::Debug for BorrowedSocket<'_> {
192 | |     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
193 | |         f.debug_struct("BorrowedSocket").field("socket", &self.socket).finish()
195 | | }
    | |_^

error: implementation has missing stability attribute
error: implementation has missing stability attribute
   --> library/std/src/os/windows/io/socket.rs:197:1
    |
197 | / impl fmt::Debug for OwnedSocket {
198 | |     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
199 | |         f.debug_struct("OwnedSocket").field("socket", &self.socket).finish()
201 | | }
    | |_^

error: associated function has missing stability attribute
error: associated function has missing stability attribute
   --> library/std/src/os/windows/io/socket.rs:207:5
    |
207 |     fn as_socket(&self) -> BorrowedSocket<'_>;

error: implementation has missing stability attribute
error: implementation has missing stability attribute
   --> library/std/src/os/windows/io/socket.rs:226:1
    |
226 | / impl AsSocket for BorrowedSocket<'_> {
227 | |     #[inline]
228 | |     fn as_socket(&self) -> BorrowedSocket<'_> {
229 | |         *self
231 | | }
    | |_^

error: implementation has missing stability attribute
