diff --git a/src/librustc_back/target/x86_64_unknown_linux_gnu.rs b/src/librustc_back/target/x86_64_unknown_linux_gnu.rs
    index f95bcb5..dd93595 100644
    --- a/src/librustc_back/target/x86_64_unknown_linux_gnu.rs
    +++ b/src/librustc_back/target/x86_64_unknown_linux_gnu.rs
    @@ -15,6 +15,7 @@ pub fn target() -> TargetResult {
         base.cpu = "x86-64".to_string();
         base.max_atomic_width = Some(64);
         base.pre_link_args.push("-m64".to_string());
    +    base.has_elf_tls = false;
    
         Ok(Target {
             llvm_target: "x86_64-unknown-linux-gnu".to_string(),
    