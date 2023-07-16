rust
pub fn target_feature_to_llvm_feature(s: &str) -> String {
    match s {
        "pclmulqdq" => "pclmul".to_string(),
        s => s.to_string()
    }
}
