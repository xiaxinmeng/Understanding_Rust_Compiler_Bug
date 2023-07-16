 rust
pub trait MyHash<S: Writer = SipHasher> for Sized? {
    fn myhash<'a>(&'a self, state: &mut S, msg: &mut Option<&'a [u8]>);
}

pub fn myhash<T: MyHash<SipHasher>>(value: &T) -> u64 {
    let mut state = SipState::new();
    let mut msg = None;
    value.myhash(&mut state, &mut msg);
    if let Some(tail_msg) = msg {
        state.write(tail_msg);
    };
    state.finish()
}

// for primitive integer types
// impl<S: Writer> MyHash<S> for $ty
fn myhash<'a>(&'a self, state: &mut S, msg: &mut Option<&'a [u8]>) {
    if (*self as $ty).to_le() == *self as $ty { unsafe {
        let a = slice::from_raw_buf(mem::transmute(&self), mem::size_of::<$ty>());
        let a_ptr = a.as_ptr();

        match msg {
            &Some(ref mut m) if a_ptr == m.as_ptr().offset(m.len() as int) => {
                *msg = slice::from_raw_buf(mem::transmute(&m.as_ptr()),
                                           m.len() + a.len());
            }
            _ => {
                if let Some(msg_part) = *msg { state.write(msg_part); }
                *msg = Some(a);
            }
        }
    } } else {
        let a: [u8, ..::core::$ty::BYTES] = unsafe {
            mem::transmute((*self as $ty).to_le() as $ty)
        };
        if let Some(msg_part) = *msg { state.write(msg_part); }
        state.write(a.as_slice());
        *msg = None;
    }
}
