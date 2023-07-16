rust
struct Foo;

impl Foo {
    async fn foo(&self, fails: i32) -> anyhow::Result<()> {
        (fails == 0)
            .then_some(())
            .ok_or_else(|| anyhow::anyhow!("Failed synchronizing {fails} exceptions."))
            .and_then(|_| self.commit_exceptions())
    }

    async fn commit_exceptions(&self) {}
}
