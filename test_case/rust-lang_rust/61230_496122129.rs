
10:45:55|~/projects/rust/tt-parser/src/liballoc|ub-comment✓
λ rg into_box\( -F
vec.rs
638:            buf.into_box()

raw_vec.rs
696:    pub unsafe fn into_box(self) -> Box<[T]> {

boxed.rs
402:            from_boxed_utf8_unchecked(buf.into_box())
553:            buf.into_box()
876:        return unsafe { new.into_box() };
885:            unsafe fn into_box(self) -> Box<[T]> {
888:                raw.into_box()

11:03:17|~/projects/rust/tt-parser/src/liballoc|ub-comment⚡*
λ cd ../libstd/

11:05:06|~/projects/rust/tt-parser/src/libstd|ub-comment⚡*
λ rg into_box\( -F
ffi/os_str.rs
344:        let rw = Box::into_raw(self.inner.into_box()) as *mut OsStr;
663:        let rw = Box::into_raw(s.inner.into_box()) as *mut OsStr;

sys_common/os_str_bytes.rs
117:    pub fn into_box(self) -> Box<Slice> {
160:    pub fn into_box(&self) -> Box<Slice> {

sys_common/wtf8.rs
358:    pub fn into_box(self) -> Box<Wtf8> {
640:    pub fn into_box(&self) -> Box<Wtf8> {

sys/windows/os_str.rs
112:    pub fn into_box(self) -> Box<Slice> {
113:        unsafe { mem::transmute(self.inner.into_box()) }
153:    pub fn into_box(&self) -> Box<Slice> {
154:        unsafe { mem::transmute(self.inner.into_box()) }
