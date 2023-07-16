 rs
$ ack --ignore-dir llvm --ignore-dir test 'time.*u64' src/
src/libtime/lib.rs
71:        pub fn mach_absolute_time() -> u64;
145:        let ns_since_1601 = ((time.dwHighDateTime as u64 << 32) |
146:                             (time.dwLowDateTime  as u64 <<  0)) / 10;
175:pub fn precise_time_ns() -> u64 {
179:    fn os_precise_time_ns() -> u64 {
195:    fn os_precise_time_ns() -> u64 {
204:            time * TIMEBASE.numer as u64 / TIMEBASE.denom as u64
209:    fn os_precise_time_ns() -> u64 {

src/libcollections/vec.rs
2390:        b.bytes = (times * src_len) as u64;

src/librbml/io.rs
187:        b.bytes = (times * len) as u64;

src/libnative/io/file_windows.rs
216:    fn set_timeout(&mut self, _t: Option<u64>) {}
217:    fn set_read_timeout(&mut self, _t: Option<u64>) {}
218:    fn set_write_timeout(&mut self, _t: Option<u64>) {}
480:        created: stat.st_ctime as u64,
481:        modified: stat.st_mtime as u64,
482:        accessed: stat.st_atime as u64,
510:pub fn utime(p: &CString, atime: u64, mtime: u64) -> IoResult<()> {

src/libnative/io/pipe_windows.rs
270:    pub fn connect(addr: &CString, timeout: Option<u64>) -> IoResult<UnixStream> {
546:    fn set_timeout(&mut self, timeout: Option<u64>) {
551:    fn set_read_timeout(&mut self, timeout: Option<u64>) {
554:    fn set_write_timeout(&mut self, timeout: Option<u64>) {
736:    fn set_timeout(&mut self, timeout: Option<u64>) {

src/libnative/io/process.rs
127:    fn set_timeout(&mut self, timeout: Option<u64>) {

src/libnative/io/util.rs
57:pub fn ms_to_timeval(ms: u64) -> libc::timeval {
64:pub fn ms_to_timeval(ms: u64) -> libc::timeval {
104:                       timeout_ms: u64) -> IoResult<()> {
151:             timeout: u64) -> libc::c_int {
164:             timeout: u64) -> libc::c_int {

src/libnative/io/mod.rs
167:                   timeout: Option<u64>)
193:                    timeout: Option<u64>) -> IoResult<Box<rtio::RtioPipe + Send>> {
256:    fn fs_utime(&mut self, src: &CString, atime: u64,
257:                mtime: u64) -> IoResult<()> {

src/libnative/io/net.rs
239:                   timeout: Option<u64>) -> IoResult<TcpStream> {
380:    fn set_timeout(&mut self, timeout: Option<u64>) {
385:    fn set_read_timeout(&mut self, timeout: Option<u64>) {
388:    fn set_write_timeout(&mut self, timeout: Option<u64>) {
648:    fn set_timeout(&mut self, timeout: Option<u64>) {
866:    fn set_timeout(&mut self, timeout: Option<u64>) {
871:    fn set_read_timeout(&mut self, timeout: Option<u64>) {
874:    fn set_write_timeout(&mut self, timeout: Option<u64>) {

src/libnative/io/pipe_unix.rs
81:           timeout: Option<u64>) -> IoResult<Inner> {
126:                   timeout: Option<u64>) -> IoResult<UnixStream> {
196:    fn set_timeout(&mut self, timeout: Option<u64>) {
201:    fn set_read_timeout(&mut self, timeout: Option<u64>) {
204:    fn set_write_timeout(&mut self, timeout: Option<u64>) {
307:    fn set_timeout(&mut self, timeout: Option<u64>) {

src/libnative/io/file_unix.rs
183:    fn set_timeout(&mut self, _t: Option<u64>) {}
184:    fn set_read_timeout(&mut self, _t: Option<u64>) {}
185:    fn set_write_timeout(&mut self, _t: Option<u64>) {}
446:    fn mktime(secs: u64, nsecs: u64) -> u64 { secs * 1000 + nsecs / 1000000 }
462:        created: mktime(stat.st_ctime as u64, stat.st_ctime_nsec as u64),
463:        modified: mktime(stat.st_mtime as u64, stat.st_mtime_nsec as u64),
464:        accessed: mktime(stat.st_atime as u64, stat.st_atime_nsec as u64),
494:pub fn utime(p: &CString, atime: u64, mtime: u64) -> IoResult<()> {

src/libstd/io/fs.rs
805:pub fn change_file_times(path: &Path, atime: u64, mtime: u64) -> IoResult<()> {

src/libstd/io/net/udp.rs
186:    pub fn set_timeout(&mut self, timeout_ms: Option<u64>) {
194:    pub fn set_read_timeout(&mut self, timeout_ms: Option<u64>) {
202:    pub fn set_write_timeout(&mut self, timeout_ms: Option<u64>) {

src/libstd/io/net/tcp.rs
118:            io.tcp_connect(addr, Some(timeout.num_milliseconds() as u64)).map(TcpStream::new)
228:    pub fn set_timeout(&mut self, timeout_ms: Option<u64>) {
245:    pub fn set_read_timeout(&mut self, timeout_ms: Option<u64>) {
272:    pub fn set_write_timeout(&mut self, timeout_ms: Option<u64>) {
444:    pub fn set_timeout(&mut self, ms: Option<u64>) { self.obj.set_timeout(ms); }

src/libstd/io/net/unix.rs
77:            let s = io.unix_connect(&path.to_c_str(), Some(timeout.num_milliseconds() as u64));
109:    pub fn set_timeout(&mut self, timeout_ms: Option<u64>) {
117:    pub fn set_read_timeout(&mut self, timeout_ms: Option<u64>) {
125:    pub fn set_write_timeout(&mut self, timeout_ms: Option<u64>) {
212:    pub fn set_timeout(&mut self, timeout_ms: Option<u64>) {

src/libstd/io/process.rs
539:    pub fn set_timeout(&mut self, timeout_ms: Option<u64>) {

src/libstd/io/mem.rs
564:        b.bytes = (times * len) as u64;

src/librustrt/rtio.rs
192:                   timeout: Option<u64>) -> IoResult<Box<RtioTcpStream + Send>>;
200:                    timeout: Option<u64>) -> IoResult<Box<RtioPipe + Send>>;
224:    fn fs_utime(&mut self, src: &CString, atime: u64, mtime: u64) ->
248:    fn set_timeout(&mut self, timeout: Option<u64>);
264:    fn set_timeout(&mut self, timeout_ms: Option<u64>);
265:    fn set_read_timeout(&mut self, timeout_ms: Option<u64>);
266:    fn set_write_timeout(&mut self, timeout_ms: Option<u64>);
290:    fn set_timeout(&mut self, timeout_ms: Option<u64>);
291:    fn set_read_timeout(&mut self, timeout_ms: Option<u64>);
292:    fn set_write_timeout(&mut self, timeout_ms: Option<u64>);
318:    fn set_timeout(&mut self, timeout: Option<u64>);
328:    fn set_timeout(&mut self, timeout_ms: Option<u64>);
329:    fn set_read_timeout(&mut self, timeout_ms: Option<u64>);
330:    fn set_write_timeout(&mut self, timeout_ms: Option<u64>);
339:    fn set_timeout(&mut self, timeout: Option<u64>);

src/librustuv/uvio.rs
143:    fn tcp_connect(&mut self, addr: rtio::SocketAddr, timeout: Option<u64>)
259:    fn fs_utime(&mut self, path: &CString, atime: u64, mtime: u64)
293:    fn unix_connect(&mut self, path: &CString, timeout: Option<u64>)

src/librustuv/timeout.rs
112:    pub fn set_timeout(&mut self, ms: Option<u64>,
235:        mut self, obj: T, timeout: Option<u64>, io: &mut UvIoFactory,

src/librustuv/timer.rs
53:    pub fn start(&mut self, f: uvll::uv_timer_cb, msecs: u64, period: u64) {

src/librustuv/file.rs
248:    pub fn utime(loop_: &Loop, path: &CString, atime: u64, mtime: u64)
276:        fn to_msec(stat: uvll::uv_timespec_t) -> u64 {

src/librustuv/process.rs
276:    fn set_timeout(&mut self, timeout: Option<u64>) {

src/librustuv/net.rs
202:                   timeout: Option<u64>) -> Result<TcpWatcher, UvError> {
305:    fn set_timeout(&mut self, timeout: Option<u64>) {
310:    fn set_read_timeout(&mut self, ms: Option<u64>) {
322:    fn set_write_timeout(&mut self, ms: Option<u64>) {
477:    fn set_timeout(&mut self, ms: Option<u64>) {
793:    fn set_timeout(&mut self, timeout: Option<u64>) {
798:    fn set_read_timeout(&mut self, ms: Option<u64>) {
820:    fn set_write_timeout(&mut self, ms: Option<u64>) {

src/librustuv/pipe.rs
89:    pub fn connect(io: &mut UvIoFactory, name: &CString, timeout: Option<u64>)
175:    fn set_timeout(&mut self, timeout: Option<u64>) {
180:    fn set_read_timeout(&mut self, ms: Option<u64>) {
192:    fn set_write_timeout(&mut self, ms: Option<u64>) {
315:    fn set_timeout(&mut self, ms: Option<u64>) {
