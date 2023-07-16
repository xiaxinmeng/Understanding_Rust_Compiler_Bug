rust
/// Obtains an Instant using seconds and an adjustment in nanoseconds since '1970-01-01 00:00:00.000000000Z'.
///
/// # Parameters
///  - `epoch_seconds`: the seconds since the epoch.
///  - `nano_adjustment`: the adjustment amount from the given second.
///
/// # Panics
/// - if the adjusted amount of seconds would overflow the instant.
pub const fn of_epoch_second_and_adjustment(
    epoch_seconds: i64,
    nano_adjustment: i64,
) -> Instant {
    match Instant::of_epoch_second_and_adjustment_checked(epoch_seconds, nano_adjustment) {
        Some(i) => i,
        _ => panic!("nano adjustment would overflow instant"),
    }
}
