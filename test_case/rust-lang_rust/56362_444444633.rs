rust
#[cfg(target_pointer_width = "16")]
type usize16 = usize;
#[cfg(not(target_pointer_width = "16"))]
type usize16 = u16;

#[cfg(target_pointer_width = "32")]
type usize32 = usize;
#[cfg(not(target_pointer_width = "32"))]
type usize32 = u32;

#[cfg(target_pointer_width = "64")]
type usize64 = usize;
#[cfg(not(target_pointer_width = "64"))]
type usize64 = u64;
