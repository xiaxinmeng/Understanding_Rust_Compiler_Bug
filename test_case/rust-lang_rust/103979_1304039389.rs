rust
#[cfg(feature = "aarch64-softmmu")]
pub const QEMU_SYSTEM_AARCH64: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/install",
    "/bin",
    "/qemu-system-aarch64"
));

#[cfg(feature = "alpha-softmmu")]
pub const QEMU_SYSTEM_ALPHA: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/install",
    "/bin",
    "/qemu-system-alpha"
));

#[cfg(feature = "arm-softmmu")]
pub const QEMU_SYSTEM_ARM: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/install",
    "/bin",
    "/qemu-system-arm"
));

#[cfg(feature = "avr-softmmu")]
pub const QEMU_SYSTEM_AVR: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/install",
    "/bin",
    "/qemu-system-avr"
));

#[cfg(feature = "cris-softmmu")]
pub const QEMU_SYSTEM_CRIS: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/install",
    "/bin",
    "/qemu-system-cris"
));

#[cfg(feature = "hppa-softmmu")]
pub const QEMU_SYSTEM_HPPA: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/install",
    "/bin",
    "/qemu-system-hppa"
));

#[cfg(feature = "i386-softmmu")]
pub const QEMU_SYSTEM_I386: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/install",
    "/bin",
    "/qemu-system-i386"
));

#[cfg(feature = "loongarch64-softmmu")]
pub const QEMU_SYSTEM_LOONGARCH64: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/install",
    "/bin",
    "/qemu-system-loongarch64"
));

#[cfg(feature = "m68k-softmmu")]
pub const QEMU_SYSTEM_M68K: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/install",
    "/bin",
    "/qemu-system-m68k"
));

#[cfg(feature = "microblazeel-softmmu")]
pub const QEMU_SYSTEM_MICROBLAZEEL: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/install",
    "/bin",
    "/qemu-system-microblazeel"
));

#[cfg(feature = "microblaze-softmmu")]
pub const QEMU_SYSTEM_MICROBLAZE: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/install",
    "/bin",
    "/qemu-system-microblaze"
));

#[cfg(feature = "mips64el-softmmu")]
pub const QEMU_SYSTEM_MIPS64EL: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/install",
    "/bin",
    "/qemu-system-mips64el"
));

#[cfg(feature = "mips64-softmmu")]
pub const QEMU_SYSTEM_MIPS64: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/install",
    "/bin",
    "/qemu-system-mips64"
));

#[cfg(feature = "mipsel-softmmu")]
pub const QEMU_SYSTEM_MIPSEL: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/install",
    "/bin",
    "/qemu-system-mipsel"
));

#[cfg(feature = "mips-softmmu")]
pub const QEMU_SYSTEM_MIPS: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/install",
    "/bin",
    "/qemu-system-mips"
));

#[cfg(feature = "nios2-softmmu")]
pub const QEMU_SYSTEM_NIOS2: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/install",
    "/bin",
    "/qemu-system-nios2"
));

#[cfg(feature = "or1k-softmmu")]
pub const QEMU_SYSTEM_OR1K: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/install",
    "/bin",
    "/qemu-system-or1k"
));

// #[cfg(feature = "ppc64-softmmu")]
// pub const QEMU_SYSTEM_PPC64: &[u8] = include_bytes!(concat!(
//     env!("OUT_DIR"),
//     "/install",
//     "/bin",
//     "/qemu-system-ppc64"
// ));
