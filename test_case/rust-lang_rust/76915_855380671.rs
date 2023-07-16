rust
pub union SocketAncillaryBuf<const N: usize> {
    // same trick as suggested in cmsg(3) man page
    _align: cmsghdr,
    buf: [u8; N],
}

pub struct SocketAncillarySlice<'a> {
    // *buf is always aligned to cmsghdr, because we only export constructors that ensure that.
    buf: &'a mut [u8],
}

impl<const N: usize> SocketAncillaryBuf<N> {
    pub fn as_slice(&mut self) -> SocketAncillarySlice {
        unsafe { SocketAncillarySlice::from_aligned_slice(&mut self.buf) }
    }
}

impl<'a> SocketAncillarySlice<'a> {
    pub unsafe fn from_aligned_slice(buf: &'a mut [u8]) -> Self {
        Self{buf}
    }

    /// Warning: will shrink buf to fit alignment requirements if necessary.
    pub fn from_unaligned_slice(buf: &'a mut [u8]) -> Self {
        // adjust pointer of buf
    }
}
