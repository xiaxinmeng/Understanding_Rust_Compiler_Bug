
#[no_stack_check]
pub fn put(head: &mut Cell, s: &str) {
    let st: &[u8] = unsafe {utils::transmute(s)};
    unsafe {r_write((head as *mut Cell), st)};
}

#[no_stack_check]
unsafe fn r_write(head: *mut Cell, s: &[u8]) {
    match s {
        [x, xs..] => {
            (*head).incr();
            put_char(&*head, (x as char));
            r_write(head, xs)
        },
        _ => (),
    }
}
