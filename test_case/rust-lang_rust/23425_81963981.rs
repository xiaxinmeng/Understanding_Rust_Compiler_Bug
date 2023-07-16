 rust
pub struct sockaddr_storage {
   pub ss_family: sa_family_t,
   pub __ss_align: u32,
   pub __ss_pad2: [u8; 120],
}
