rust
extern "Rust" {
    #[lang = "oom"]
    fn oom_impl(layout: Layout) -> !;
}
