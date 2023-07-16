rust
use anyhow

// V1
let stdout = command.read_stdout()
    .with_context(|| format!("failed to run {:?}", comand))?;

// V2
let stdout = command.output()
    .with_context(|| format!("failed to run {:?}", comand))?
    .utf8_stdout()
    .with_context(|| format!("failed to run {:?}", comand))?;
