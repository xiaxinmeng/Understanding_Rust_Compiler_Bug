rust
enum SocketAncillaryData {
    FDs(Vec<RawFD>),
    Cred(Vec<(u32, u32, u32)>),
    SELinux(String)
}

fn recv_with_ancillary_data(
    &self, 
    bufs: &mut [IoSliceMut<'_>], 
    msg_controllen: usize
) -> (usize, Vec<SocketAncillaryData>, SocketAddr);
