rust
fn macos_default_deployment_target(arch: &str) -> (u32, u32) {
    if arch == "arm64" { (11, 0) } else { (10, 7) }
}
