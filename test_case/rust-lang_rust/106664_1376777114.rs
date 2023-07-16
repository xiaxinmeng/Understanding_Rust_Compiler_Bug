console
openat(AT_FDCWD, "./p/f.rs", O_RDONLY|O_CLOEXEC) = 3
statx(0, NULL, AT_STATX_SYNC_AS_STAT, STATX_ALL, NULL) = -1 EFAULT (Bad address)
statx(3, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|0x1000, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=88, ...}) = 0
read(3, "fn main() {\n    let res = std::f"..., 88) = 88
read(3, "", 32)                         = 0
close(3)
