rust
trait Selectable : Sized {
    fn get_fd(&self) -> RawFd;
    fn on_read(self, _: &mut Server<Self>) -> Option<Self>;
    fn on_write(self, _: &mut Server<Self>) -> Option<Self>;
    fn on_except(self, _: &mut Server<Self>) -> Option<Self>;
}

struct Server<T: Selectable>{
    readfds:VecDeque<Box<T>>,
    writefds:VecDeque<Box<T>>,
    exceptfds:VecDeque<Box<T>>,
}
