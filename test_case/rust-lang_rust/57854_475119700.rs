rust
struct PollEvented<E> {
    phantom: std::marker::PhantomData<E>,
}

impl<'a, E> std::io::Read for &'a PollEvented<E>
where
    &'a E: std::io::Read,
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        unimplemented!()
    }
}

fn bar<R>(r: &R) {
    std::io::Read::read_exact(&mut r, &mut [])
    //std::io::copy(&mut r, unimplemented!())
    //std::io::BufReader::new(r)
}
