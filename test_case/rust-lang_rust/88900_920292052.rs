rust
#[derive(Debug)]
struct MyEvent {
    pub message: String,
    pub random_value: f32,
}

fn receiving_system(mut event_reader: EventReader<MyEvent>) {
    for my_event in event_reader.iter() {
        log::debug!("processing {:?}", *my_event);
        do_work(my_event)
    }
}
