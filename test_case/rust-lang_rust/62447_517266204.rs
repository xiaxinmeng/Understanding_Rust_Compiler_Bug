
$cat aarch64_alpine_linux_musl.rs 

use crate::spec::TargetResult;

pub fn target() -> TargetResult {
    let base = super::aarch64_unknown_linux_musl::target()?;

    Ok(base)
}
