
324     if unsafe { rust_try_get_task().is_not_null() } {
325         return OldTaskContext;
326     } else {
327         if Local::exists::<Scheduler>() {
328             let context = Cell::new_empty();
