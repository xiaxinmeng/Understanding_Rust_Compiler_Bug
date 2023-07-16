 rust
pub struct Command {
    inner: /*target-specific module*/::Command,
    stdin: Option<Stdio>,
    stdout: Option<Stdio>,
    stderr: Option<Stdio>,
}
