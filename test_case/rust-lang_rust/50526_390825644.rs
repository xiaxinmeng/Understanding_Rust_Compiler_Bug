
fn set_frames<
  Frame,
  Walk: FnMut(c::DWORD, c::HANDLE, c::HANDLE, &mut Frame, &mut c::CONTEXT)
  Addr: Fn(&mut Frame) -> *const u8,
>(frames: &mut [Frame], walk: Walk, addr: Addr) -> io::Result<usize>
