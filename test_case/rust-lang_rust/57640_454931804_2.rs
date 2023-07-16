
async fn try_log(message: String) -> Result<usize, Error> {
    let logger = acquire_lock()(?);
    // Very terse, construct the future, wait on it, branch on its result.
    let length = logger.log_into(message)(?)?;
    logger.timestamp()(?);
    Ok(length)
}
