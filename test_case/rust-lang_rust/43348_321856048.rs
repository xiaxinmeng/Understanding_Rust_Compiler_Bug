rust
#[cfg(not(unix))]
mod libc {
    pub use libc::c_int;
    pub type socklen_t = u32;
    pub struct sockaddr;
    #[derive(Clone)]
    pub struct sockaddr_un;
}
