rust
fn spawn_synchronisation_task(
    daemon: PompaDaemon,
    location: crate::models::Location,
    expected_dates: Vec<Range>,
) {
    tokio::task::spawn(daemon.sync_data_for(location, expected_dates));
}

