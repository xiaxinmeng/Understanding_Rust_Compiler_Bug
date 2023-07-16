rust
#[test]
fn test_mitigation(){
    use std::os::fortanix_sgx::arch::MITIGATED_ADVISORIES;
    assert!(MITIGATED_ADVISORIES.contains(&"INTEL-SA-00079"));
    assert!(MITIGATED_ADVISORIES.contains(&"INTEL-SA-00076"));
}
