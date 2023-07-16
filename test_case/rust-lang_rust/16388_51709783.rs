 rust
struct stack_t {
    ss_sp: *mut libc::c_void,
    ss_flags: libc::c_int,
    ss_size: libc::size_t    
}
extern {
    fn sigaltstack(ss: *const stack_t, oss: *mut stack_t) -> libc::c_int
}
