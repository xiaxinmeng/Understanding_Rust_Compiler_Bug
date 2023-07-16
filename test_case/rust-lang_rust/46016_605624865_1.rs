rust
/// This should be called before calling any cli method or printing any output.
pub fn reset_signal_pipe_handler() -> Result<()> {
    #[cfg(target_family = "unix")]
    {
        use nix::sys::signal;

        unsafe {
            signal::signal(signal::Signal::SIGPIPE, signal::SigHandler::SigDfl)
                .map_err(|e| Error::Other(e.to_string()))?;
        }
    }

    Ok(())
}
