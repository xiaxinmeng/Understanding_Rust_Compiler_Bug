
*self = unsafe { mem::transmute(self.slice_from_mut(write_len)) };
