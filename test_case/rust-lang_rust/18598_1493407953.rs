rust
/// The command we are executing
#[derive(Discriminant)]
enum Cmd {
    /// a zsh script to execute
    Zsh,
    Bash,
}

#[async_trait]
trait Command {
    async fn execute(&self, exec: Ctx, input: &str) -> anyhow::Result<String>;
}

#[async_trait]
impl Command for Zsh {
    async fn execute(&self, exec: Ctx, input: &str) -> anyhow::Result<String> {
        // ...
    }
}

#[async_trait]
impl Command for Bash {
    async fn execute(&self, exec: Ctx, input: &str) -> anyhow::Result<String> {
        // ...
    }
}

fn this_requires_unsize() {
    let cmd1: Box<dyn Command> = Cmd::Zsh.cast();
    let cmd2: Box<dyn Command> = Cmd::Bash.cast();
}
