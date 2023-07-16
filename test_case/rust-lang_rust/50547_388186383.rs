rust
pub async fn get_statuses() -> Vec<StatusUpdate> {
    // get_status_updates is also an `async fn`, but calling it works just like any other call:
    let count = get_status_updates();

    let mut tasks = vec![];
    for i in 0..count {
        // Here is where task *construction* becomes explicit, as an async block:
        task.push(async {
            // Again, simply *calling* get_status_update looks just like a sync call:
            let status_update: StatusUpdateStruct = get_status_update(i).deserialize();
            StatusUpdate::new(utc_from_ticks(status_update.update_date), status_update.status_code, status_update.note))
        });
    }

    // And finally, launching the explicitly-constructed futures is also explicit, while awaiting the result is implicit:
    join_all(&tasks[..])
}
