rust
loop {
    if let Ok(x) = self.try_recv_ref() {
        unsafe {
            return &*(x as *const T);
        }
     }
}
