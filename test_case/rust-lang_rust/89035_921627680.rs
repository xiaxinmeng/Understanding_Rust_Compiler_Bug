
rustc 1.54.0 (a178d0322 2021-07-26) running on x86_64-unknown-linux-gnu
compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental -C link-arg=-fuse-ld=lld --crate-type lib

// error origin differs by 4 lines, I guess something was added there: 
/rustc/a178d0322ce20e33eac124758e837cbd80a6f633/compiler/rustc_query_system/src/query/plumbing.rs:620:9

// rustc complains about blanket implementation similar to this:
{
    use async_trait::async_trait;
    
    pub trait SomeTrait: SomeOtherTrait {..}
    
    #[async_trait]
    impl SomeTrait for CustomStruct {}
}
