rust
fn is_unwindsafe(_: impl std::panic::UnwindSafe) {}

fn main() {
    is_unwindsafe(async {
        // this needs an inner await point
        async {}.await;
    });
}
