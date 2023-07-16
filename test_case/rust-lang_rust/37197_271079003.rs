raw
let fake_rc: Self = mem::transmute(ptr);
let fake_rc_target = fake_rc.as_ref() as *const _ as *const u8;
