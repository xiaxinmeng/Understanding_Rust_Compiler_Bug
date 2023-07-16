
$ rg ': Into<' src/lib{std,alloc,core}
src/libcore/convert.rs
262:/// fn is_hello<T: Into<Vec<u8>>>(s: T) {
569:impl<T, U> TryFrom<U> for T where U: Into<T> {
617:/// impl<T, U> TryFrom<U> for T where U: Into<T> {

src/libstd/process.rs
694:    pub fn stdin<T: Into<Stdio>>(&mut self, cfg: T) -> &mut Command {
720:    pub fn stdout<T: Into<Stdio>>(&mut self, cfg: T) -> &mut Command {
746:    pub fn stderr<T: Into<Stdio>>(&mut self, cfg: T) -> &mut Command {

src/libstd/net/addr.rs
569:impl<I: Into<IpAddr>> From<(I, u16)> for SocketAddr {

src/libstd/io/error.rs
250:        where E: Into<Box<dyn error::Error+Send+Sync>>

src/libstd/ffi/c_str.rs
323:    pub fn new<T: Into<Vec<u8>>>(t: T) -> Result<CString, NulError> {
