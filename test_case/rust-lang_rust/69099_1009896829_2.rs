rust
// Modified and shortened obviously..
fn run(addr: &RefCell<OrdSlice<Addr>>) -> Result<()> {
    let mut state = Network::new(addr);
    let mut queues = …;

    loop {
        ingress(&mut state, &mut queues);
        egress(&mut state, &mut queues);

        queues.errors()?;
    }

    Ok(())
}

fn main() {
    let addr: OrdSlice<_, [_; 16]> = Default::default();
    let mut addr = RefCell::new(addr);

    loop {
        // Otherwise, second `run` will panic.
        RefCell::undo_leak(&mut addr);

        if let Err(_) = run(&addr) { … }
    }
}
