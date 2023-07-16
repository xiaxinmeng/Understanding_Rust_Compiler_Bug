 rust
loop {
        // Wait for events to arrive
        let event = ev_queue.next_event().unwrap(); 
        event.callback_fn(event);
}
