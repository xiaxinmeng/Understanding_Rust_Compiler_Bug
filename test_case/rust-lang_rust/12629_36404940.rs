 rust
use std::libc;
use std::io;

struct AbortingWriter(bool);

impl Writer for AbortingWriter {
    fn write(&mut self, buf: &[u8]) -> io::IoResult<()> {
        let ret = io::stderr().write(buf);
        let AbortingWriter(ref mut written) = *self;
        *written = true;
        return ret;
    }
}

impl Drop for AbortingWriter {
    fn drop(&mut self) {
        let AbortingWriter(written) = *self;
        if written {
            unsafe { libc::exit(1) }
        }
    }
}

fn main() {
    spawn(proc() {
        io::stdio::set_stderr(~AbortingWriter(false));
        fail!("oh no!");
    })
}
