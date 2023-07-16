rust
struct SocketAncillaryData {
   // opaque type
}

impl SocketAncillaryData {
  pub fn new(capacity: usize) -> Self

  // some of the data was lost due to insufficient capacity
  // retrieved via MSG_CTRUNC flag from recvmsg
  pub fn truncated() -> bool

  pub fn get_fds(fds: &mut [usize]) -> usize
}


fn recv_with_ancillary_data(
    &self, 
    bufs: &mut [IoSliceMut<'_>], 
    ancilliary: &mut SocketAncillaryData 
) -> Result<(usize, SocketAddr), ...>;
