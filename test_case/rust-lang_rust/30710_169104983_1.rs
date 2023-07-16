 Rust
use target::Target;

pub fn target() -> Target {
    Target {
        llvm_target: "arm-unknown-linux-gnueabi".to_string(),
        target_endian: "little".to_string(),
        target_pointer_width: "32".to_string(),
        arch: "arm".to_string(),
        target_os: "linux".to_string(),
        target_env: "gnueabi".to_string(),
        target_vendor: "unknown".to_string(),
        options: super::linux_base::opts()
    }
}
