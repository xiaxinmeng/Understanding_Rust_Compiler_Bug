
trait Writer {
    fn write(&mut self, buf: &[u8]);
}

trait Flushable : Writer {
    fn flush(&mut self);
}

struct IgnoreFlush<T>(T);

impl<T: Writer> Writer for T {
    fn write(&mut self, buf: &[u8]) {
        (**self).write(buf)
    }
}

impl<T: Writer> Flushable for T {
     fn flush(&mut self) { /* do nothing */ }
}
