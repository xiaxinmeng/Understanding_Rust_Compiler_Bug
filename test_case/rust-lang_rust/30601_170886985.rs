
pub struct sigaction {
    pub sa_flags: ::c_int,
    ...
}
...

pub const SA_ONSTACK: ::c_uint = 0x08000000;
pub const SA_SIGINFO: ::c_uint = 0x00000008;
