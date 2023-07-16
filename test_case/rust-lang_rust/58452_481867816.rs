rust
pub struct IoVec<'a>(…);
pub struct IoVecMut<'a>(…);

impl<'a> fmt::Debug for IoVec<'a> {…}
impl<'a> fmt::Debug for IoVecMut<'a> {…}

impl<'a> IoVec<'a> {
    pub fn new(buf: &'a [u8]) -> Self {…}
}
impl<'a> IoVecMut<'a> {
    pub fn new(buf: &'a mut [u8]) -> Self {…}
}

impl<'a> Deref for IoVec<'a> {
    type Target = [u8];
}
impl<'a> Deref for IoVecMut<'a> {
    type Target = [u8];
}
impl<'a> DerefMut for IoVecMut<'a> {…}

pub trait Write {
    fn write_vectored(&mut self, bufs: &[IoVec<'_>]) -> Result<usize> {…}
}
pub trait Read {
    fn read_vectored(&mut self, bufs: &mut [IoVecMut<'_>]) -> Result<usize> {…}
}